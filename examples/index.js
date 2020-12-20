// For more comments about what's going on here, check out the `hello_world`
// example
import('./pkg')
  .catch(console.error);


function drawCanvas() {
  const canvas = document.getElementById("#canvas");
  console.log(canvas);
}

async function getData() {
  const data = (await import('./pkg')).getImageData().get_data()
  return data
}



let canvas = document.createElement('canvas');
canvas.id = "canvas";
canvas.width = 150
canvas.height = 150

getData().then(data => {
  const ctx = canvas.getContext('2d')
  console.log(canvas.width)
  const imageData = ctx.createImageData(canvas.width, canvas.height)
  imageData.data.set(data)
  console.log('put data', data, 'size', data.length)
  ctx.putImageData(imageData, 0, 0);
})


document.body.appendChild(canvas);
