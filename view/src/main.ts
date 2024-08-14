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

	// purge all cached events when the page is not visible
	document.addEventListener("visibilitychange", () => {
		state.leave();
	});
	document.addEventListener("blur", () => {
		state.leave();
	});

	// `update` is called 60 times per second
	const updateInterval = 1000 / 60;
	const initialTime = Date.now();

	const updateloop = () => {
		const currentTime = Date.now();
		state.update((currentTime - initialTime) / updateInterval);
		const nextTime = Date.now();

		const passedTime = nextTime - currentTime;
		const remainingTime = Math.max(0, updateInterval - passedTime);
		setTimeout(updateloop, remainingTime);
	};

	updateloop();
}

window.onload = main;
