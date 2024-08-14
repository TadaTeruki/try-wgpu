import init, { State } from "../pkg/graphics.js";

async function main() {
	await init();

	const canvas = document.getElementById("main-canvas") as HTMLCanvasElement;

	const fullscreenOnlyCanvas = () => {
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
	};
	fullscreenOnlyCanvas();

	const state = await new State(canvas);

	const fullscreen = () => {
		fullscreenOnlyCanvas();
		state.resize(window.innerWidth, window.innerHeight);
	};

	addEventListener("resize", fullscreen);
	fullscreen();

	const update = () => {
		state.render();
		requestAnimationFrame(update);
	};

	update();
}

window.onload = main;
