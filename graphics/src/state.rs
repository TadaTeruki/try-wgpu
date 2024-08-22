use cgmath::InnerSpace;
use wasm_bindgen::prelude::*;
use wgpu::{
    util::DeviceExt, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    RenderPipelineDescriptor, SamplerBindingType, ShaderStages, SurfaceTarget,
};

use crate::{
    camera::{geometry::CameraGeometry, perspective::CameraPerspective, Camera},
    fetch::Fetcher,
    key::{KeyState, KeyStateMap},
    light::{
        self,
        property::{LightProperty, LightVertex},
        Light,
    },
    model::{
        model::{DrawModel, Model},
        vertex::ModelVertex,
    },
    star::{Star, StarInstanceRaw},
};

#[wasm_bindgen]
pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    key_states: KeyStateMap,
    camera: Camera,
    light: Light,
    earth_render_pipeline: wgpu::RenderPipeline,
    earth_model: Model,
    sky_render_pipeline: wgpu::RenderPipeline,
    star_render_pipeline: wgpu::RenderPipeline,
    star: Star,
    light_render_pipeline: wgpu::RenderPipeline,
}

#[wasm_bindgen]
impl State {
    pub(crate) async fn new(
        canvas: web_sys::HtmlCanvasElement,
        use_gl_instead: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let width = canvas.width();
        let height = canvas.height();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface_target = SurfaceTarget::Canvas(canvas);
        let surface = instance.create_surface(surface_target)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: if use_gl_instead {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await?;

        let surface_caps: wgpu::SurfaceCapabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(
                surface_caps
                    .formats
                    .first()
                    .copied()
                    .expect("No surface formats"),
            )
            .add_srgb_suffix();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: surface_caps
                .present_modes
                .first()
                .copied()
                .expect("No present modes"),
            alpha_mode: surface_caps
                .alpha_modes
                .first()
                .copied()
                .expect("No alpha modes"),
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let earth_radius = 500.0;

        let perspective = CameraPerspective::new(
            CameraGeometry::new(
                (earth_radius * 4.0, 0.0, 0.0).into(),
                (0.0, 0.0, 0.0).into(),
                cgmath::Vector3::unit_y(),
            ),
            50.0,
            config.width as f32 / config.height as f32,
            45.0,
            0.1,
            100.0,
        );
        let camera = Camera::new(&device, perspective);

        let distance_between_light_and_earth = 11728.0 * earth_radius * 2.0;
        let earth_axis = cgmath::Vector3::new(0.0, 0.398, 0.917).normalize();

        let light_property = LightProperty::new(
            (
                0.0,
                distance_between_light_and_earth * earth_axis.y,
                distance_between_light_and_earth * earth_axis.z,
            )
                .into(),
            (1.0, 1.0, 1.0).into(),
        );

        let light = Light::new(&device, light_property);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("texture_bind_group_layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let href = web_sys::window().unwrap().location().href().unwrap();
        let fetcher = Fetcher::new(&href);

        let blend_state = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::Zero,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        };

        let sky_render_pipeline = {
            let shader = device.create_shader_module(wgpu::include_wgsl!("shader/sky.wgsl"));
            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("sky_render_pipeline_layout"),
                    bind_group_layouts: &[&camera.bind_group_layout],
                    push_constant_ranges: &[],
                });

            let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("sky_render_pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(blend_state),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });

            render_pipeline
        };

        let (star, star_render_pipeline) = {
            let shader = device.create_shader_module(wgpu::include_wgsl!("./shader/star.wgsl"));
            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("star_render_pipeline_layout"),
                    bind_group_layouts: &[&camera.bind_group_layout],
                    push_constant_ranges: &[],
                });

            let star = Star::new(&device, &queue);

            let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("star_render_pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[StarInstanceRaw::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(blend_state),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });

            (star, render_pipeline)
        };

        let (earth_model, earth_render_pipeline) = {
            let (earth_obj, earth_mtl, earth_texture_diffuse) = futures::join!(
                fetcher.fetch_as_bytes("resources/earth/earth.obj"),
                fetcher.fetch_as_bytes("resources/earth/earth.mtl"),
                fetcher.fetch_as_bytes("resources/earth/earth_diff.png"),
            );

            let (earth_obj, earth_mtl, earth_texture_diffuse) = futures::join!(
                earth_obj?.bytes(),
                earth_mtl?.bytes(),
                earth_texture_diffuse?.bytes(),
            );

            let (earth_obj, earth_mtl, earth_texture_diffuse) = (
                &earth_obj? as &[u8],
                &earth_mtl? as &[u8],
                &earth_texture_diffuse? as &[u8],
            );

            let model = Model::create(
                &device,
                &queue,
                earth_obj,
                earth_mtl,
                earth_texture_diffuse,
                &texture_bind_group_layout,
            )
            .await?;

            let shader = device.create_shader_module(wgpu::include_wgsl!("shader/earth.wgsl"));

            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("earth_render_pipeline_layout"),
                    bind_group_layouts: &[
                        &camera.bind_group_layout,
                        &texture_bind_group_layout,
                        &light.uniform_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });

            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("earth_render_pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[ModelVertex::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            });

            (model, render_pipeline)
        };

        let light_render_pipeline = {
            let shader = device.create_shader_module(wgpu::include_wgsl!("shader/light.wgsl"));
            let render_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("light_render_pipeline_layout"),
                    bind_group_layouts: &[&camera.bind_group_layout],
                    push_constant_ranges: &[],
                });

            device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("light_render_pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[LightVertex::desc()],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(blend_state),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                cache: None,
            })
        };

        Ok(Self {
            surface,
            device,
            queue,
            config,
            key_states: KeyStateMap::new(),
            camera,
            light,
            earth_render_pipeline,
            earth_model,
            sky_render_pipeline,
            star_render_pipeline,
            star,
            light_render_pipeline,
        })
    }

    #[wasm_bindgen]
    pub fn key_event(&mut self, event: &web_sys::KeyboardEvent) {
        let released = event.type_() == "keyup";
        let key = event.key();
        if released {
            self.key_states.insert(key, KeyState::Release);
        } else {
            if self.key_states.get(&key).is_some() {
                return;
            }
            self.key_states.insert(key, KeyState::Press);
        }
    }

    #[wasm_bindgen]
    pub fn leave(&mut self) {
        self.key_states.purge();
    }

    #[wasm_bindgen]
    pub async fn update(&mut self, _time: f32) {
        self.camera.perspective.process_events(&self.key_states);
        self.queue.write_buffer(
            &self.camera.buffer,
            0,
            bytemuck::cast_slice(&[self.camera.perspective.build_uniform()]),
        );
        self.key_states.update();
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.camera
            .perspective
            .update_aspect(width as f32 / height as f32);
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.269 / 255.0,
                            g: 0.388 / 255.0,
                            b: 0.342 / 255.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.star_render_pipeline);
            render_pass.set_vertex_buffer(0, self.star.instance_buffer.slice(..));
            render_pass.set_bind_group(0, &self.camera.bind_group, &[]);
            render_pass
                .set_index_buffer(self.star.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(
                0..self.star.num_indices,
                0,
                0..self.star.instances.len() as u32,
            );

            render_pass.set_pipeline(&self.light_render_pipeline);
            render_pass.set_vertex_buffer(0, self.light.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.camera.bind_group, &[]);
            render_pass.draw(0..3, 0..1);

            render_pass.set_pipeline(&self.earth_render_pipeline);
            render_pass.draw_model(
                &self.earth_model,
                &self.camera.bind_group,
                &self.light.uniform_bind_group,
            );

            render_pass.set_pipeline(&self.sky_render_pipeline);
            render_pass.set_bind_group(0, &self.camera.bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
