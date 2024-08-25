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
	const gpuNotSupported = !navigator.gpu;
	if (gpuNotSupported) {
		console.error("WebGPU not supported");
		const loading = document.getElementById("loading");
		if (loading) {
			loading.innerHTML = `Sorry, your browser does not support WebGPU.<br>
			See <a href='https://caniuse.com/webgpu' target="_blank">caniuse.com</a>
			to check the current status of WebGPU support.
			`;
		}
		return;
	}

	// create state
	const state = await create_state(canvas, false);
	if (!state) {
		return;
	}

	// resize
	addEventListener("resize", () => fullscreenCanvas(canvas, state));
	fullscreenCanvas(canvas, state);

	// `render` is called per the refresh rate of the display
	const renderloop = () => {
		state.render();
		requestAnimationFrame(renderloop);
	};
	requestAnimationFrame(renderloop);

	// key event
	addEventListener("keydown", (event) => {
		state.key_event(event);
	});
	addEventListener("keyup", (event) => {
		state.key_event(event);
	});

	// reset all events when the page is not visible
	document.addEventListener("visibilitychange", () => {
		state.leave();
	});
	document.addEventListener("blur", () => {
		state.leave();
	});

	const allowLeft = document.getElementById("allow-left");
	const allowRight = document.getElementById("allow-right");

	let isAllowLeft = false;
	allowLeft?.addEventListener("mousedown", () => {
		isAllowLeft = true;
	});
	allowLeft?.addEventListener("mouseup", () => {
		isAllowLeft = false;
	});
	allowLeft?.addEventListener("mouseleave", () => {
		isAllowLeft = false;
	});
	allowLeft?.addEventListener("touchstart", () => {
		isAllowLeft = true;
	});
	allowLeft?.addEventListener("touchend", () => {
		isAllowLeft = false;
	});

	let isAllowRight = false;
	allowRight?.addEventListener("mousedown", () => {
		isAllowRight = true;
	});
	allowRight?.addEventListener("mouseup", () => {
		isAllowRight = false;
	});
	allowRight?.addEventListener("mouseleave", () => {
		isAllowRight = false;
	});
	allowRight?.addEventListener("touchstart", () => {
		isAllowRight = true;
	});
	allowRight?.addEventListener("touchend", () => {
		isAllowRight = false;
	});

	// `update` is called 60 times per second
	const updateInterval = 1000 / 60;
	const initialTime = Date.now();

	const updateloop = () => {
		const currentTime = Date.now();
		state.update((currentTime - initialTime) / updateInterval);
		if (isAllowLeft) {
			state.scroll_to_left();
		}
		if (isAllowRight) {
			state.scroll_to_right();
		}
		const nextTime = Date.now();

		const passedTime = nextTime - currentTime;
		const remainingTime = updateInterval - passedTime;
		if (remainingTime < 0) {
			console.log("update");
			updateloop();
		} else {
			setTimeout(updateloop, remainingTime);
		}
	};

	updateloop();
}

window.onload = main;
