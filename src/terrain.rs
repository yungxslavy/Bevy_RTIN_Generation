use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};
use bevy::pbr::wireframe::{Wireframe};

use self::heightmaps::{generate_heightmap_data, generate_vertices, get_mapsize};
mod heightmaps;

#[derive(Reflect, Resource, Default, Component)]
#[reflect(Resource)]
pub struct MyMesh{
    pub verts: u32,
    pub index: u32,
}

pub fn create_mesh_chunk(path: String) -> Mesh{

    // Create our initial mesh component using triangle list primitive topology 
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // Hold our current plane for shifting 
    let indices = generate_vertices(path.clone());
    let mut planechunk = convert_indices_to_f32x3(indices, path.clone());
    let tri = planechunk.len();

    apply_heightmap(&mut planechunk, path.clone());

    // Positions of the vertices
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        planechunk,
    );

    // Normals and UV 
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; tri]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; tri]);

    // A triangle using vertices 0, 2, and 1.
    // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
    mesh.set_indices(Some(mesh::Indices::U32(getindexcount(tri as u32))));

    return mesh
} 

fn convert_indices_to_f32x3(indices: Vec<f32>, path: String) -> Vec<[f32; 3]>{

    let map_size = get_mapsize(path) as f32;

    let mut new_indices = Vec::new();
    for i in 0..indices.len(){
        let x = indices[i] % map_size;
        let z = (indices[i] / map_size).floor();
        let y = 0.0; 

        new_indices.push([x / map_size, y, z / map_size]);
    }
    return new_indices;
}



// Usually you wont need to go more than 4-5 levels in detail 
pub fn _generateplanevertices(size: f32, detail: u32) -> Vec<[f32; 3]>{

    // Get our variables we need to make triangles 
    let magic: i32 = 4; // Can't exponentiate an integer literal (?? rust sux)
    let squares = magic.pow(detail); // # of squares the entire chunk has been composed to 
    let squares_inside = (squares as f64).sqrt() as i32; // # of squares in side length
    let mini_sq_len = size / squares_inside as f32; 

    let mut verts = Vec::new(); // Holds all of the triangle vertices 

    for i in 0..squares_inside{
        for j in 0..squares_inside{
            // Triangle 1 
            verts.push([i as f32 * mini_sq_len, 0., j as f32 * mini_sq_len]);
            verts.push([(i as f32 * mini_sq_len) + mini_sq_len, 0., j as f32 * mini_sq_len]);
            verts.push([(i as f32 * mini_sq_len) + mini_sq_len, 0., (j as f32 * mini_sq_len) + mini_sq_len]);

            // Triangle 2
            verts.push([i as f32 * mini_sq_len, 0., j as f32 * mini_sq_len]);
            verts.push([(i as f32 * mini_sq_len) + mini_sq_len, 0., (j as f32 * mini_sq_len) + mini_sq_len]);
            verts.push([(i as f32 * mini_sq_len), 0., (j as f32 * mini_sq_len) + mini_sq_len]);
        }
    }

    return verts;
}

pub fn getindexcount(num: u32) -> Vec<u32>{
    let mut vect: Vec<u32> = Vec::new();
    let mut  _counter = 0; 

    // We need this wonky system to make index 
    // in the form of 0, 2, 1, 3, 5, 4 to make the shape show
    // for x in 0..num{
    //     if counter == 1 {
    //         counter += 1;
    //         vect.push(x + 1)
    //     }
    //     else if counter == 2{
    //         counter = 0; 
    //         vect.push(x - 1); 
    //     }
    //     else {
    //         counter += 1;
    //         vect.push(x)
    //     }
    // }

    // Same as above but now flipped 
    for x in 0..num{
        vect.push(x);
    }

    return vect;
}

// This function is used to apply a heightmap to the terrain by modifying the y value of each vertex
// based on the heightmap data that is passed in.
fn apply_heightmap (verts: &mut Vec<[f32; 3]>, map_path: String,){
    // Get the heightmap data
    let map_data = generate_heightmap_data(map_path.clone());
    let map_size = map_data.len() as f32 - 1.0;
    let height_scale = 0.5;

    // Iterate through each vertex and modify the y value based on the heightmap data
    for vert in verts.iter_mut(){
        let x = vert[0];
        let z = vert[2];

        // Get the index of the heightmap data that corresponds to the vertex
        let x_to_index = (x * map_size) as usize;
        let z_to_index = (z * map_size) as usize;

        if z_to_index >= map_data.len() || x_to_index >= map_data.len(){
            println!("{} {} {}", x, z, map_data.len());
        }

        // Get the y value from the heightmap data and set the vertex y value to it
        let yval = map_data[x_to_index as usize][z_to_index as usize] * height_scale;
        vert[1] = yval;
    }
}

pub fn change_mesh(
    mouse_button: Res<Input<MouseButton>>,
    key_button: Res<Input<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,  
    mut query: Query<(Entity, &MyMesh)>,
) {
    // This function runs when the left mouse button is pressed and will change the mesh of the entity
    // by spawning a new entity with the new mesh and despawning the old one
    if mouse_button.just_pressed(MouseButton::Middle) {

        let texture_handle: Handle<Image> = asset_server.load("grass.png");
        
        // this material renders the texture normally
        let _material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        });

        let maps = vec!["./maps/height.png", "./maps/mountain.png", "./maps/team.png"];
        let mut choice = 0; 

        // Despawn the old entity
        for (ent, my_mesh) in query.iter_mut(){
            // Get the next map in the list
            if my_mesh.index >= maps.len() as u32 - 1{
                choice = 0;
            }            
            else{
                choice = my_mesh.index + 1;
            }

            // Rid the entity of its mesh
            commands.entity(ent).despawn();
        }

        // Spawn a new entity with the new mesh
        let mesh = create_mesh_chunk(maps[choice as usize].to_string());

        // Check if the shift key is pressed to spawn the entity with wireframe
        if key_button.pressed(KeyCode::LShift){
            commands.spawn(
                (PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::rgb(0., 0., 0.).into()),
                ..default()
            },
            MyMesh {index: choice, verts: 0}));
        }
        else{
            commands.spawn((PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::rgb(0., 0., 0.).into()),
                ..default()
            },
            Wireframe,
            MyMesh {index: choice, verts: 0}));
        }
    }
}