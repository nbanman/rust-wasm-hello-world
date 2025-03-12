# wasm-game-of-life Example w/ Vite

The Rust and WebAssembly project lets developers write type-safe Rust targeting WASM.

Unfortunately, the [Game Of Life Tutorial](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html) no longer compiles [^1] [^2] [^3] [^4] [^5].

Thanks to https://github.com/rustwasm/create-wasm-app/issues/215, I was able to get a working "Hello World" working with Vite.

[^1]: https://github.com/rustwasm/create-wasm-app/issues/215
[^2]: https://github.com/rustwasm/wasm-bindgen/issues/4251
[^3]: https://github.com/rustwasm/wasm-bindgen/issues/4211#issue-2598192238
[^4]: https://github.com/rustwasm/rust-webpack-template/issues/191
[^5]: https://github.com/rustwasm/wasm-bindgen/discussions/4171