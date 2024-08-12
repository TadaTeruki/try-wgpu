export const utils = {
	autoResizeCanvas(canvas) {
		const expandFullScreen = () => {
			canvas.width = window.innerWidth;
			canvas.height = window.innerHeight;
		};
		expandFullScreen();
		window.addEventListener("resize", expandFullScreen);
	},
};
