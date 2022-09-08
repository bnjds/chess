use bevy::prelude::*;
use bevy_mod_picking::*;
use std::f32::consts::PI;

mod pieces;
use pieces::*;

mod board;
use board::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Chess Game".to_string(),
            width: 1000.,
            height: 600.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin) //Remove this to get rid of the green debug line/sphere
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_startup_system(setup_system) //startup_systems only run once, at the beginning
        .run();
}


fn setup_system(
    mut commands: Commands,
) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                               Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                               Vec3::new(-2.0, 10.0, 4.0),
            )),
            ..default()
        })
    .insert_bundle(PickingCameraBundle::default());
    commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 25000.,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
            ..default()
        });
}

