import init, { create_state, type State } from "../pkg/graphics.js";

function fullscreenCanvas(
	canvas: HTMLCanvasElement,
	state: State | undefined = undefined,
) {
	canvas.width = window.innerWidth;
	canvas.height = window.innerHeight;
	if (state) {
		state.resize(window.innerWidth, window.innerHeight);
	}
}

async function main() {
	// initialize
	const canvas = document.getElementById("main-canvas") as HTMLCanvasElement;
	fullscreenCanvas(canvas);
	await init();

	// if the navigator does not have field 'gpu' then use WebGL
	const useGlInstead = !navigator as any["gpu"];
	if (useGlInstead) {
		console.error("WebGPU not supported, falling back to WebGL");
	}

	// create state
	const state = await create_state(canvas, useGlInstead);
	if (!state) {
		return;
	}

	// resize
	addEventListener("resize", () => fullscreenCanvas(canvas, state));
	fullscreenCanvas(canvas, state);

	// mainloop
	const update = () => {
		state.render();
		requestAnimationFrame(update);
	};

	update();
}

window.onload = main;
