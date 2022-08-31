use bevy::prelude::*;

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
        .run();
}
