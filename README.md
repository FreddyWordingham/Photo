# 🌟 `Photo`: Precision Ray-Tracing

<p align="center">
  <img src="./resources/images/banner.png" alt="Photo logo" width="350" height="200">
</p>

## 🔭 Overview

`Photo` is a Rust-built ray-tracing library emphasizing precision and modularity, aimed at accurate light simulation for photorealism. It uniquely leverages CPU capabilities, enhancing hardware compatibility and precision.

🔗 GPU Adaptation: [Photo GPU Branch](https://github.com/FreddyWordingham/Photo/tree/gpu).

## 🚀 Key Features

- 🖥️ **High Precision & Parallelism**: Optimized for multi-core CPUs. It features tile-based rendering, reducing memory usage and enabling render checkpointing.
- 🎨 **Advanced Rendering Techniques**: Foregoing traditional textures, `Photo` uses colourmaps and detailed shadow calculations for lifelike images.
- 🌐 **Flexible Scene Description**: Offers JSON-configured scenes and supports .obj files for model inputs, providing extensive creative liberty.

Dive into `Photo` for a unique, CPU-focused ray-tracing experience that redefines photorealistic rendering.

## 🏎️ Supports

- Ray-tracing
- Tile-based rendering
- Temporal rendering
- Boundary volume hierarchy
- Mesh instancing
- Surfaces types:
  - Opaque
  - Reflective
  - Refractive
  - Emissive

## 🏁 Quickstart

Clone the repository and set the root folder as the current working directory:

```shell
git clone https://github.com/FreddyWordingham/photo.git photo
cd photo
```

Build the package using Cargo:

```shell
cargo build --release
```

And then run one of the binary, targetting an input parameters file:

```shell
cargo run --release ./input/parameters.json
```

## 📖 Details

`Photo` is a ray-tracing library written in Rust, with a strong focus on precision and modularity.
It is designed to be highly precise, with a focus on the physically accurate simulation of light transport through a scene, to produce photorealistic images.

It uses the CPU rather than the GPU to perform the ray tracing, allowing it to be run on a wide range of hardware and to a higher degree of precision than a GPU would allow.
(See the [GPU](https://github.com/FreddyWordingham/Photo/tree/gpu) branch for an implementation of the library which targets the GPU.)

Although the library targets the CPU, it is designed to be highly parallelised, allowing it to take advantage of multi-core processors.
The image is rendered in an array of tiles which minimises the memory footprint, and allows for checkpointing of the render.

Rather than using textures, the `Photo` uses colourmaps and shadow calculations to produce a photorealistic image.
Scene descriptions are written in JSON, and input models are wavefront (.obj) files, allowing for a high degree of flexibility in the scene description.
