# raytracing
A rust implimentation of Raytracing in a weekend
https://raytracing.github.io/books/RayTracingInOneWeekend.html

## How to use
No extra requirements to run, however it is highly recommended to compile in
release mode to speed up the program if using it for larger renders. Program
will output to a file called image.jpg in the current working directory. 

## The Scene
The program generates a triangle mesh to raytrace against by using a perlin
noise function as a height map.

## Working Features
1. Spheres
2. Materials [Lambertian, Metal]
3. Lighting & Shadows
4. Movable Camera
5. Triangles
6. Procedural Terrain
    * Height Map
    * Colour Map
