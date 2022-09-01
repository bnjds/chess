use bevy::prelude::*;
use std::f32::consts::PI;

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
        });
    commands
        .spawn_bundle(DirectionalLightBundle {
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
            ..default()
        });
}


fn create_board_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
    //meshes.add(....): Registers a plane mesh to the Assets<Mesh> resource, and return a Handle<Mesh>, which is what PbrBundle uses.
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));

    //do the same but for materials ( <StandardMaterial> )
    let white_material = materials.add(Color::rgb(1., 0.9, 0.9).into());
    let black_material = materials.add(Color::rgb(0., 0.1, 0.1).into());

    //spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                //change material according to position to get alternating pattern
                material: if (i + j + 1) % 2 == 0 {
                    white_material.clone()
                } else {
                    black_material.clone()
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..default()
            });
        }
    }
}



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
        .add_startup_system(setup_system) //startup_systems only run once, at the beginning
        .add_startup_system(create_board_system)
        .run();
}
