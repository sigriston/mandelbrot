# mandelbrot
Mandelbrot set computation written in Rust, compiled to wasm

## Instructions

1. Install Rust nightly and the `wasm32-unknown-unknown` target:
```console
curl https://sh.rustup.rs -sSf | sh
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

2. Install [wasm-gc]:
```console
rustup run nightly cargo install --git https://github.com/alexcrichton/wasm-gc
```

3. Install node.js & yarn

4. Clone this repo

5. Install packages and run:
```console
yarn
yarn start
```

6. Direct your browser to http://localhost:8080 and enjoy!

[wasm-gc]: https://github.com/alexcrichton/wasm-gc
