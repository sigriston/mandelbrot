# mandelbrot
Mandelbrot set computation written in Rust, compiled to wasm

## Instructions

1. Install Rust nightly and the `wasm32-unknown-unknown` target:

```console
curl https://sh.rustup.rs -sSf | sh
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

1. Install node.js & yarn

1. Clone this repo

1. Install packages and run:

```console
yarn
yarn start
```

1. Direct your browser to http://localhost:8080 and enjoy!
