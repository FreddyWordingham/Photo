# Photo

<p align="center">
    <img src="./resources/images/icon.svg" alt="Photo Icon" width="200" height="200">
</p>

Render scenes with simulations.

## Quickstart

If you have the [`Rust`](https://www.rust-lang.org/) toolchain installed, you can download the latest release using `cargo`:

```shell
cargo install photo
```

## Source

Alternatively, you can build the project from source.

### Dependencies

-   [`Rust`](https://www.rust-lang.org/).
-   [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) (If you're building for the web).

```shell
git clone https://github.com/FreddyWordingham/Photo.git Photon
cd Photon
```

First, clone the repository and then set the current working directory to the root of the project:

### Desktop

Compile the project binaries using `cargo`:

```shell
cargo build --release
```

After which you can run the binaries.

### Web

Compile the project binaries using `wasm-pack`:

```shell
wasm-pack build --target web --release
```

You can then import the compiled WASM module into your web project:

```javascript
const init = await import("./pkg/photo_bg.js");
init().then(() => console.log("Photo WASM module Loaded."));
```

See [index](./index.html).

```shell
open index.html
```
