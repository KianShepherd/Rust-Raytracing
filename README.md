# Rust-Raytracing
A Rust implementation of [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html). 
It currently implements all functionality of the book excluding Dielectrics which are a WIP. I have also added the code 
for ray triangle intersection as well as procedural terrain generation with triangle meshes and perlin noise height 
mapping. Simple lighting has also been created by sending a ray back towards the light source. Multi-threading has also 
been added and can optionally be turned off with a flag within the main function.


![Example Render1](https://github.com/KianShepherd/Rust-Raytracing/blob/master/example1.jpg?raw=true)

![Example Render2](https://github.com/KianShepherd/Rust-Raytracing/blob/master/example2.jpg?raw=true)

![Example Render3](https://github.com/KianShepherd/Rust-Raytracing/blob/master/example3.jpg?raw=true)

## Dependencies
* Rand  = "*"
* Noise = "0.7"
* Image = "*"
* num_cpus = "1.6"

## How to use
It is highly recommended compiling in release mode to speed up the program if using it for larger renders. Once compiled
simply supply the path to the .ron file within the /configs/ folder, and an image will be output in the main directory with
the same name.

## Features
1. Materials 
    * Lambertian
    * Metal
    * Dielectric (IN PROGRESS)
    * Mirror
2. Lighting
    * Multiple Point Lights
    * Shadows
3. Camera
    * Movable
    * Defocus Blur
4. Shapes
    * Spheres
    * Triangles (with optional back face culling)
    * 2D Squares
    * 3D Cube (IN PROGRESS)
5. Procedural Terrain
    * Height Map
    * Colour Map
6. Multi-Threading
   * Benchmarks
     * Release mode on 8 Core CPU
     * 720p procedural gen
         * multi-threaded: 0h : 14m : 27s
         * single thread: 1h : 7m : 35s
         * 4.7x speedup
     * 400p test scene
         * multi-threaded: 0m : 19s
         * single thread: 1m : 51s
         * 5.8x speedup
7. Config Files
   * Raytracer settings loaded from a file to make creating multiple images with similar settings easy.
   * allow specifying of settings file from command line (optionally multiple).
8. Octree / K-tree Optimization (TODO)