import("./pkg").catch(console.error);

import { ImageRawData } from "./pkg";

interface Point {
  x: number;
  y: number;
}

async function getData(width: number, height: number, point: Point) {
  const data = (await import("./pkg")).ImageRawData.get_image(
    width,
    height,
    point === undefined ? { x: -1000, y: -1000 } : point,
  );
  return data;
}

function getOptions() {
  const form = document.getElementById("form") as HTMLFormElement;
  const formData = new FormData(form);

  const width = Number(formData.get("width") || 300);
  const height = Number(formData.get("height") || 300);
  return {
    width,
    height,
  };
}

function drawToCanvas(data: ImageRawData) {
  const canvas = document.getElementById("canvas") as HTMLCanvasElement;
  const ctx = canvas.getContext("2d");
  console.log(canvas.width);
  canvas.width = data.get_width();
  canvas.height = data.get_height();
  console.log(data.width, data.height);
  const imageData = ctx.createImageData(canvas.width, canvas.height);
  const dataArray = data.get_data();
  imageData.data.set(dataArray);
  console.log("put data", data, "size", dataArray.length);
  ctx.putImageData(imageData, 0, 0);

  console.log("done drawing");
}

const form = document.getElementById("form");
form.addEventListener("submit", async (event) => {
  event.preventDefault();
  const options = getOptions();
  const data = await getData(options.width, options.height, undefined);
  drawToCanvas(data);
});

const canvas = document.getElementById("canvas");
canvas.addEventListener("mousedown", async (e: MouseEvent) => {
  const target = e.target as Element;
  const rect = target.getBoundingClientRect();
  const x = e.clientX - rect.left;
  const y = e.clientY - rect.top;
  console.log("pos", x, y);

  const options = getOptions();
  const data = await getData(options.width, options.height, { x, y });
  drawToCanvas(data);
});

window.onload = (_) => {
  console.log(getOptions());
  const options = getOptions();
  getData(options.width, options.height, undefined).then((d) =>
    drawToCanvas(d),
  );
};
