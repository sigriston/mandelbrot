# mandelbrot
Mandelbrot set computation written in Rust, compiled to wasm

## Instructions

1. Install Rust nightly and the `wasm32-unknown-unknown` target:
```console
curl https://sh.rustup.rs -sSf | sh
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

2. Install node.js & yarn

3. Clone this repo

4. Install packages and run:
```console
yarn
yarn start
```

5. Direct your browser to http://localhost:8080 and enjoy!
