# Photo

Rendering engine using Rust.
Deliberately targets the CPU for extended render times and high-precision images.

## Quickstart

Clone the repository and run the following commands:

```bash
git clone https://github.com/FreddyWordingham/photo.git
cd photo
cargo build --release
```

Then run the executable targeting the scene file you wish to render:

```bash
cargo run --release input/scene.yaml
```
