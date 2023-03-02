use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::AnglePositions;

// Import the camera movement script
mod camera;
mod terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_plugin(WorldInspectorPlugin)
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup)
        .add_system(camera::camera_movement_system)
        .add_system(camera::rotate_camera_system)
        .add_system(terrain::change_mesh)
        .run();
}

fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // To draw the wireframe on all entities, set this to 'true'
    wireframe_config.global = false;

    // Spawn a new entity with the new mesh
    let mesh = terrain::create_mesh_chunk("./maps/height.png".to_string());

    commands.spawn((
        PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0., 0., 0.).into()),
        ..default()
        }, 
        terrain::MyMesh {index: 0, verts: 0},
        Wireframe,)
    );
   
    // spawn a small red sphere 
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1,
            subdivisions: 4,
        })),
        material: materials.add(Color::rgb(1., 0., 0.).into()),
        transform: Transform::from_xyz(0.0, -0.5, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });


    commands.spawn(Camera3dBundle {
        ..default()
    });

    commands.insert_resource(AnglePositions {yaw: 0.0, pitch: 0.0});
}