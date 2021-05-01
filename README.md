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
There are no requirements to run this program that are not included in 
the Cargo.toml, however it is highly recommended to compile in release 
mode to speed up the program if using it for larger renders. Program
will output to a file called image.jpg in the current working directory. 

A few scripts have been included to compile in Debug/Release mode as well 
as clean up the /target directory and generated image. These are made to
compile and run the program then they will attempt to display it using 
feh. If feh is not present on your system you can either edit the script
to open the image in the image viewer of your choice or simply run the 
program and open the generated image afterwords.

```
./release.sh    or    cargo run --release
```

## The Scene
The program generates a procedural triangle mesh terrain to raytrace against 
by using a perlin noise function as a height map. It also maps different 
colours to the triangles based on height. It also includes 3 spheres to help
demonstrate the lightings shadow effect.

## Working Features
1. Materials 
    * Lambertian
    * Metal
    * Dielectric (IN PROGRESS)
    * Mirror (TODO)
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