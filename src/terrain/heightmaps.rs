use image;
use image::{GenericImageView, Pixel}; 


pub fn generate_heightmap_data(path: String) -> Vec<Vec<f32>>{
    // Load the image
    let img = image::open(path).unwrap();

    // Convert the image to grayscale
    let grayscale = img.grayscale();

    // Get the dimensions of the image
    let (width, height) = grayscale.dimensions();

    // Convert the grayscale values to heights
    let mut heightmap = vec![vec![0.0; width as usize]; height as usize];
    for y in 0..height {
        for x in 0..width {
            let pixel = grayscale.get_pixel(x, y);
            let value = pixel.channels()[0] as f32 / 255.0;
            heightmap[y as usize][x as usize] = value;
        }
    }

    return heightmap;
}


pub fn generate_vertices(path: String) -> Vec<f32>{
    // Load the image
    let img = image::open(path).unwrap();

    // Convert the image to grayscale
    let grayscale = img.grayscale();

    // Get the dimensions of the image
    let (width, height) = grayscale.dimensions();

    // Convert the grayscale values to heights
    let mut terrain = vec![0.0; width as usize * height as usize];
    let mut count = 0;

    // Create the map in a single array that will be used to determine the height of each pixel 
    for y in 0..height{
        for x in 0..width{
            // Determine the height of the pixel based on the grayscale value
            let pixel = grayscale.get_pixel(x, y);
            let value = pixel.channels()[0] as f32 / 255.0;
            
            // *IMPORTANT* The value will be normalized to [0.0, 1.0] so you must amplify it to match the desired height
            terrain[count] = value * 1000.0;
            count += 1;
        }
    }
    
    // Generate the error map to let the process triangle function know which triangles to split
    let errors = generate_error_map(terrain, width, width - 1);

    // These will hold the new vertices of our triangle points 
    let mut indices: Vec<f32> = Vec::new(); 

    // This is the maximum error we will allow before splitting a triangle
    // e.g if the difference in height between two points is greater than this value, then we will split the triangle
    // Play around with this value to get the desired result
    let max_error = 10.0; 

    // Recursively split the triangles until the last triangle is reached. We do this twice for both the top and bottom triangles
    process_triangle(&mut indices, max_error, &errors, width, 0, 0, width - 1, width - 1, width - 1, 0);
    process_triangle(&mut indices, max_error, &errors, width, width - 1, width - 1, 0, 0, 0, width - 1);

    // Return those vertices that we gathered from the map. 
    return indices;
}


pub fn get_mapsize(path: String) -> u32{
    // Load the image
    let img = image::open(path).unwrap();

    // Get the dimensions of the image
    let height= img.height();

    return height;
}


// This function generates the error map we will be using 
fn generate_error_map(terrain: Vec<f32>, grid_size: u32, tile_size: u32) -> Vec<f32>{
    // This holds the error values for each point
    let mut errors = vec![0.0; grid_size as usize * grid_size as usize];

    // Mathematical constants
    let num_smallest_triangles = tile_size * tile_size;
    let num_triangles = num_smallest_triangles * 2 - 2; 
    let last_level_index = num_triangles - num_smallest_triangles; 

    // Iterate through each triangle and calculate the error
    for i in (0..num_triangles).rev() {
        // Calculate the indices of the triangle's vertices
        let mut id = i + 2; 
        let mut ax = 0; let mut ay = 0; let mut bx = 0; let mut by = 0; let mut cx = 0; let mut cy = 0;

        if (id & 1) > 0 {
            bx = tile_size; by = tile_size; cx = tile_size;  // Bottom left triangle 
        }
        else {
            ax = tile_size; ay = tile_size; cy = tile_size; // top right triangle
        }
        
        // Iterate through the levels of the triangle
        while (id >> 1) > 1 {
            id >>= 1; 

            // Calculate the middle point of the triangle
            let mx = (ax + bx) >> 1;
            let my = (ay + by) >> 1;

            // Left half 
            if (id & 1) > 0{
                bx = ax; by = ay;
                ax = cx; ay = cy; 
            }
            // Right half
            else {
                ax = bx; ay = by; 
                bx = cx; by = cy;
            }

            cx = mx; cy = my;
        }

        // Calculate the error of the triangle
        let interpolated_height = (terrain[(ay * grid_size + ax) as usize] + terrain[(by * grid_size + bx) as usize]) / 2.0;
        let middle_index = ((ay + by) >> 1) * grid_size + ((ax + bx) >> 1);
        let middle_error = (interpolated_height - terrain[middle_index as usize]).abs();

        // If we are at the last level, set the error of the triangle
        if i >= last_level_index {
            errors[middle_index as usize] = middle_error;
        }
        else {
            // Otherwise, set the error of the triangle to the biggest error of its children
            let left_child_error = errors[(((ay + cy) >> 1) * grid_size + ((ax + cx) >> 1)) as usize];
            let right_child_error = errors[(((by + cy) >> 1) * grid_size + ((bx + cx) >> 1)) as usize];
            let options = [errors[middle_index as usize], middle_error, left_child_error, right_child_error];
            errors[middle_index as usize] = get_biggest_float(options);
        }
    }

    return errors;
}

// This function returns the biggest float in a set
fn get_biggest_float (set: [f32; 4]) -> f32 {
    let mut biggest = 0.0;
    for i in 0..set.len() {
        if set[i] > biggest {
            biggest = set[i];
        }
    }
    return biggest;
}

// This function recursively splits the triangles until the last triangle is reached
fn process_triangle(indices: &mut Vec<f32>, max_error: f32, errors: &Vec<f32>, grid_size: u32, ax: u32, ay: u32, bx: u32, by: u32, cx: u32, cy: u32){
    let mx = (ax + bx) >> 1;
    let my = (ay + by) >> 1;

    if (((ax as i32 - cx as i32).abs() + (ay as i32 - cy as i32).abs()) > 1) && (errors[(my * grid_size + mx) as usize] > max_error) {
        process_triangle(indices, max_error, errors, grid_size, cx, cy, ax, ay, mx, my);
        process_triangle(indices, max_error, errors, grid_size, bx, by, cx, cy, mx, my)
    }
    else {
        indices.push((ay * grid_size + ax) as f32);
        indices.push((by * grid_size + bx) as f32);
        indices.push((cy * grid_size + cx) as f32);
    }
}