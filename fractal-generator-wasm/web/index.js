import {MandelBrot} from "fractal-generator-wasm";
var $ = (id) => {return Number(document.getElementById(id).value)};

let animation = null;



const WIDTH = 1024;
const HEIGHT = 1024;
let zoom = $("zoom")
let zoom_point_x = $("zoom_point_x")
let zoom_point_y = $("zoom_point_y")
let transparency = 255 - $("transparency")
let iteration = $("iteration")
let zoom_factor_increase = 1
const generateButton = document.getElementById("generate");
const resetBtn = document.getElementById("reset");

const canvas = document.getElementById("fractal-canvas");
canvas.height = HEIGHT;
canvas.width = WIDTH;

const context = canvas.getContext("2d");

const isAnimationPaused = () => {
  	return animation === null
}

const playAnimation = () => {
	generateButton.textContent = "⏸ Stop";
	renderLoop();
}

const stopAnimation = () => {
	generateButton.textContent = "▶ Start";
	cancelAnimationFrame(animation)
	animation = null
}

const resetAnimation = () => {
	zoom = $("zoom")
	zoom_point_x = $("zoom_point_x")
	zoom_point_y = $("zoom_point_y")
	transparency = (255 - $("transparency"))
	iteration = $("iteration")
	mandelBrot.set_transparency(transparency);
	mandelBrot.set_iterations(iteration);
	renderLoop()
}

const mandelBrot = MandelBrot.new(canvas.width, canvas.height, iteration, transparency);

function renderLoop(){
	let measurementStart = performance.now()
	let image = mandelBrot.to_img(zoom, zoom_point_x, zoom_point_y)
	context.putImageData(image, 0, 0)
	zoom = zoom + zoom_factor_increase
	document.getElementById("performanceIndicator").innerHTML = "generated in " + (performance.now() - measurementStart) + "ms"
	animation = requestAnimationFrame(renderLoop)
}

generateButton.addEventListener("click", event => {
	if (isAnimationPaused()) {
		playAnimation();
	} else {
		stopAnimation();
	}
});

resetBtn.addEventListener("click", event => {
  	resetAnimation();
})



canvas.addEventListener("click", event => {
	if (isAnimationPaused()) {
		playAnimation();
	} else {
		stopAnimation();
	}
});

playAnimation();