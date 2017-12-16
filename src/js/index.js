(function() {
  const setupCanvas = () => {
    let canvas = document.getElementById('canvas');
    let retina = window.devicePixelRatio || 1;

    let width = canvas.width = window.innerWidth * retina;
    let height = canvas.height = window.innerHeight * retina;

    canvas.style.width = `${width / retina}px`;
    canvas.style.height = `${height / retina}px`;

    return canvas;
  };

  const getBuffer = (buffer, pointer, width, height) => {
      const byteSize = width * height * 4;
      return new Uint8ClampedArray(buffer, pointer, byteSize);
  };

  const fillCanvas = (canvas, buffer) => {
    if (canvas.getContext) {
      const { width, height } = canvas;
      let ctx = canvas.getContext('2d');
      const imageData = new ImageData(buffer, width, height);
      ctx.putImageData(imageData, 0, 0);
    }
  };

  fetch('mandelbrot.wasm')
    .then(rsp => rsp.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes, {}))
    .then(module => {
      const {
        instance: {
          exports: {
            mandelbrot,
            memory
          }
        }
      } = module;

      let canvas = setupCanvas();
      const { width, height } = canvas;

      let pointer = mandelbrot(width, height);
      let buffer = getBuffer(memory.buffer, pointer, width, height);

      fillCanvas(canvas, buffer);
    });
}());
