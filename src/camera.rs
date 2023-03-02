
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

#[derive(Resource)]
pub struct AnglePositions{
    pub yaw: f32,
    pub pitch: f32,
}


pub fn camera_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>
) {
    let speed = 4.0;

    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            let unitvec = transform.forward() * speed;
            transform.translation += unitvec * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::A) {
            let unitvec = transform.left() * speed;
            transform.translation += unitvec * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S) {
            let unitvec = transform.back() * speed;
            transform.translation += unitvec * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            let unitvec = transform.right() * speed;
            transform.translation += unitvec * time.delta_seconds();
        }
    }
}


pub fn rotate_camera_system(
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_button: Res<Input<MouseButton>>,
    mut state: ResMut<AnglePositions>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    if mouse_button.pressed(MouseButton::Right) {
        // Right Button is being held down
        for ev in mouse_motion.iter(){
            state.yaw -= (ev.delta.x as f32 * 0.25).to_radians();
            state.pitch -= (ev.delta.y as f32 * 0.25).to_radians();
        }
    }

    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_axis_angle(Vec3::Y, state.yaw) * Quat::from_axis_angle(Vec3::X, state.pitch);
    }
}