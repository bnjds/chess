use crate::pieces::*;
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


//Resource
#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>,
}

//Resource
#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>,
}


fn create_board_system(
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

//Query: provides us with an iterable of all the entities that have the Components we select, 
//  in this case Entity (which all entities have), Square, and Handle<StandardMaterial>. 
//  We can now iterate over it with query.iter(), which will provide us access to the components.
//Note: In queries, components have to be references (i.e. have &), except Entity, which is used normally.
fn color_squares_system(
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Square, &Handle<StandardMaterial>)>,
    picking_camera_query: Query<&PickingCamera>,
) {
    //TODO: READ THIS: https://github.com/aevyrie/bevy_mod_picking/issues/63 AND THIS: https://github.com/aevyrie/bevy_mod_picking/issues/112
    //get Entity under the cursor, if there is one 
    let top_entity = match picking_camera_query.iter().last() {
        Some(picking_camera) => match picking_camera.intersect_top() {
            Some((entity, _intersection)) => Some(entity),
            None => None,
        },
        None => None,
    };

    for (entity, square, material) in query.iter() {
        //get the actual material
        let material = materials.get_mut(material).unwrap();

        //change the material color 
        material.base_color = if Some(entity) == top_entity {
            Color::rgb(0.8, 0.3, 0.3)
        } else if Some(entity) == selected_square.entity {
            Color::rgb(0.9, 0.1, 0.1)
        } else if square.is_white() {
            Color::rgb(1.0, 0.9, 0.9)
        } else {
            Color::rgb(0., 0.1, 0.1)
        };
    }
}


fn select_square_system(
    picking_camera_query: Query<&PickingCamera>,
    squares_query: Query<&Square>,
    mut pieces_query: Query<(Entity, &mut Piece)>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
){
    //check if left mouse button pressed, if NO then return
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return; 
    }
    
    //get the square under the cursor and set it as the selected square
    if let Some(picking_camera) = picking_camera_query.iter().last() {
        dbg!("{:#?}", picking_camera.intersect_list());
        if let Some((square_entity, _intersection)) = picking_camera.intersect_top() {
            if let Ok(square) = squares_query.get(square_entity) {
                //mark it as selected
                selected_square.entity = Some(square_entity);
                
                if let Some(selected_piece_entity) = selected_piece.entity {
                    if let Ok((_piece_entity, mut piece)) = pieces_query.get_mut(selected_piece_entity) {
                        piece.x = square.x;
                        piece.y = square.y;
                    }
                    selected_square.entity = None;
                    selected_piece.entity = None;       
                } else {
                    //select the piece in the currently selected square
                    for(piece_entity, piece) in pieces_query.iter_mut() { 
                        if piece.x == square.x && piece.y == square.y {
                            //piece_entity is now the entity in the same square
                            selected_piece.entity = Some(piece_entity);
                            break;
                        }
                    }
                }
            }
        } else {
            //Player clicked outside the board, deselect everything 
            selected_square.entity = None; 
            selected_piece.entity = None; 
        } 
    } 
}


pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board_system)
            .add_system(color_squares_system)
            .add_system(select_square_system);
    } 
}
    
