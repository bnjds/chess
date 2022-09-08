use bevy::prelude::*;


#[derive(Component, Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum PieceType {
    King, 
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    //current position
    pub x: u8,
    pub y: u8,
}


fn move_pieces_system(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        //get the direction to move
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;
        
        //only move if the piece isnt already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}


fn create_pieces_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Load the meshes: NOTE: A handle is not the asset itself, but should be seen as a pointer to the asset
    let king_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh1/Primitive0");

    let knight_1_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh4/Primitive0");

    let pawn_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh2/Primitive0");
    let rook_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> = asset_server.load("models/pieces.glb#Mesh7/Primitive0");


    //materials
    let white_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1., 0.8, 0.8),
        perceptual_roughness: 0.9,
        ..default()
    });
    let black_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.2, 0.2, 0.2),
        perceptual_roughness: 0.9,
        ..default()
    });

    //NOTE: (official bevy) example that uses "&mut commands" as arg / for a fn that has "commands: &mut Commands" as a parameter:
    //  https://github.com/bevyengine/bevy/blob/992681b59b93be3efd52ad8d5a34ebb4ddfd0c20/examples/stress_tests/bevymark.rs
    spawn_rook(
        &mut commands, 
        white_material.clone(),
        PieceColor::White,
        rook_handle.clone(),
        (0, 0),
    );
    spawn_knight(
        &mut commands, 
        white_material.clone(),
        PieceColor::White,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (0, 1),
    );
    spawn_bishop(
        &mut commands, 
        white_material.clone(),
        PieceColor::White,
        bishop_handle.clone(),
        (0, 2),
    );
    spawn_queen(
        &mut commands, 
        white_material.clone(),
        PieceColor::White,
        queen_handle.clone(),
        (0, 3),
    );
    spawn_king(
        &mut commands, 
        white_material.clone(),
        PieceColor::White,
        king_handle.clone(),
        king_cross_handle.clone(),
        (0, 4),
    );
    spawn_bishop(
        &mut commands, 
        white_material.clone(),
        PieceColor::White,
        bishop_handle.clone(),
        (0, 5),
    );
    spawn_knight(
        &mut commands, 
        white_material.clone(),
        PieceColor::White,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (0, 6),
    );
    spawn_rook(
        &mut commands, 
        white_material.clone(),
        PieceColor::White,
        rook_handle.clone(),
        (0, 7),
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            white_material.clone(),
            PieceColor::White,
            pawn_handle.clone(),
            (1, i),
        );
    }

    spawn_rook(
        &mut commands, 
        black_material.clone(),
        PieceColor::Black,
        rook_handle.clone(),
        (7, 0),
    );
    spawn_knight(
        &mut commands, 
        black_material.clone(),
        PieceColor::Black,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (7, 1),
    );
    spawn_bishop(
        &mut commands, 
        black_material.clone(),
        PieceColor::Black,
        bishop_handle.clone(),
        (7, 2),
    );
    spawn_queen(
        &mut commands, 
        black_material.clone(),
        PieceColor::Black,
        queen_handle.clone(),
        (7, 3),
    );
    spawn_king(
        &mut commands, 
        black_material.clone(),
        PieceColor::Black,
        king_handle.clone(),
        king_cross_handle.clone(),
        (7, 4),
    );
    spawn_bishop(
        &mut commands, 
        black_material.clone(),
        PieceColor::Black,
        bishop_handle.clone(),
        (7, 5),
    );
    spawn_knight(
        &mut commands, 
        black_material.clone(),
        PieceColor::Black,
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        (7, 6),
    );
    spawn_rook(
        &mut commands, 
        black_material.clone(),
        PieceColor::Black,
        rook_handle.clone(),
        (7, 7),
    );

    for i in 0..8 {
        spawn_pawn(
            &mut commands,
            black_material.clone(),
            PieceColor::Black,
            pawn_handle.clone(),
            (6, i),
        );
    }
}


fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: (u8, u8),
) {
    //add PARENT
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
        ..default()
    })
    .insert(Piece {
        color: piece_color,
        piece_type: PieceType::King,
        x: position.0,
        y: position.1,
    })
    //add CHILDREN
    //NOTE: The closure gets 1 argument (given the name "parent"), a reference to the current item in the slice being considered
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..default()
        });
        parent.spawn_bundle(PbrBundle {
            mesh: mesh_cross,
            material,
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..default()
        });
    });
}

fn spawn_knight(
    commands: &mut Commands, 
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh_one: Handle<Mesh>,
    mesh_two: Handle<Mesh>,
    position: (u8, u8),
) {
    //add PARENT
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
        ..default()
    })
    .insert(Piece {
        color: piece_color,
        piece_type: PieceType::Knight,
        x: position.0,
        y: position.1,
    })
    //add CHILDREN
    //NOTE: The closure gets 1 argument (given the name "parent"), a reference to the current item in the slice being considered
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh: mesh_one,
            material: material.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..default()
        });
        parent.spawn_bundle(PbrBundle {
            mesh: mesh_two,
            material,
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..default()
        });
    });
}

fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    commands.spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..default()
    })
    .insert(Piece {
        color: piece_color,
        piece_type: PieceType::Queen,
        x: position.0,
        y: position.1,
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..default()
        });
    });
}

fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    commands.spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..default()
    })
    .insert(Piece {
        color: piece_color,
        piece_type: PieceType::Bishop,
        x: position.0,
        y: position.1,
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..default()
        });
    });
}

fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    commands.spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..default()
    })
    .insert(Piece {
        color: piece_color,
        piece_type: PieceType::Rook,
        x: position.0,
        y: position.1,
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..default()
        });
    });
}
        
fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    mesh: Handle<Mesh>,
    position: (u8, u8),
) {
    commands.spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(
                position.0 as f32,
                0.,
                position.1 as f32,
            )),
            ..default()
    })
    .insert(Piece {
        color: piece_color,
        piece_type: PieceType::Pawn,
        x: position.0,
        y: position.1,
    })
    .with_children(|parent| {
        parent.spawn_bundle(PbrBundle {
            mesh,
            material,
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                transform
            },
            ..default()
        });
    });
}


pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_pieces_system)
            .add_system(move_pieces_system);
    }
}
