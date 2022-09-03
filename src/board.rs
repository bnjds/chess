use bevy::prelude::*;
use bevy_mod_picking::*;



#[derive(Component)]
pub struct Square {
    pub x: u8, 
    pub y: u8,
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}



#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}



pub fn create_board_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //meshes.add(....): Registers a plane mesh to the Assets<Mesh> resource, and return a Handle<Mesh>, which is what PbrBundle uses.
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));

    //do the same but for materials ( <StandardMaterial> )

    //spawn 64 squares
    for i in 0..8 {
        for j in 0..8 {
            commands.spawn_bundle(MaterialMeshBundle {
                mesh: mesh.clone(), //new mesh is a copy of the mesh created above
                //change material according to position to get alternating pattern
                material: if (i + j + 1) % 2 == 0 {
                    materials.add(Color::rgb(1., 0.9, 0.9).into())
                } else {
                    materials.add(Color::rgb(0., 0.1, 0.1).into()) 
                },
                transform: Transform::from_translation(Vec3::new(i as f32, 0., j as f32)),
                ..default()
            })
            .insert_bundle(PickableBundle::default())
            .insert(Square { x: i, y: j});
        }
    }
}


fn color_squares(
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
) {
    //get Entity under the cursor, if there is one 
    
}
