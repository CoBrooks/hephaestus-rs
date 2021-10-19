# Hephaestus-rs

Hephaestus-rs is a work-in-progress 3D game engine written in Rust using the Vulkano wrapper around the Vulkan Graphics API. 

![Demo Screenshot](/screenshots/demo_screenshot_1.png)

## Current Features

- Full render pipeline capable of rendering multiple objects with ambient lighting and multiple directional lights of varying color, intensity, and direction.
- Programmatically generated primitive objects: Planes, Cubes, and Spheres.
- Dynamic loading of custom meshes using the Wavefront (.obj) format.
- The ability to apply textures to objects (just color textures for now).
- A detailed logging system (will be expanded upon once more user interaction with the scene is implemented).
- A debug UI that shows the debug log and a breakdown of the time it takes to render each frame.

## Running

1. Install the Rust programming language (see ![the Rust install page](https://www.rust-lang.org/tools/install)).
2. Clone the repo with `git clone https://github.com/CoBrooks/hephaestus-rs`.
3. Run `cargo run --release` inside of the root directory.

Note: This project was developed on an Arch Linux system with an AMD GPU, thus I cannot guarantee that it will work on other platforms or hardware.

## TODO

- Add the ability to move the camera around the scene.
- More lighting: Specular highlights and point lights.
- ...
