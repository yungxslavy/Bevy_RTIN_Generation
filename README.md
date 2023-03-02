# Terrain Generation Display

## Overview 

This code reads in a PNG image from the `maps` folder, grayscales it using the `heightmap.rs` file, and then generates a 3D geometry using `terrain.rs` to represent the image.

## Controls

- **Hold the Shift Key:** Modifies the next terrain to have a wireframe when clicked with the middle mouse button. 
- **Middle Mouse Button:** Toggles the next image terrain. 
- **Hold the Right Mouse Button:** Enables the ability to look around.
- **WASD:** Provides movement to the camera based on its current look vector.

## Getting Started

1. Clone or download this repository.
2. Make sure you have the necessary dependencies installed. (`cargo run` installs it too)
3. Run the code with `cargo run` to experience the terrain generation display!
4. Add your own images into the map folder, then add the path to the `change_mesh()` in `terrain.rs`! 

## Important Stuff 

- Please ensure that all of the images are sized to (2<sup>n</sup> + 1) X (2<sup>n</sup> + 1) pixels. This is so the RTIN algorithm can work appropriately.
- To mess with the parameters please change the heightmap.rs file and look for the `max_error` variable, as well as the line containing `terrain[count] = value * 1000.0`. The latter multiplies the value by a max height of your choosing. The `max_error` is the height difference the algorithm will determine is the threshold before splitting the triangle.
