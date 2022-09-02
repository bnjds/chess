use bevy::prelude::*;

pub fn spawn_king(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: Vec3,
) {
    //add PARENT
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
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

pub fn spawn_knight(
    commands: &mut Commands, 
    material: Handle<StandardMaterial>,
    mesh_one: Handle<Mesh>,
    mesh_two: Handle<Mesh>,
    position: Vec3,
) {
    //add PARENT
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_translation(position),
        ..default()
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

pub fn spawn_queen(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands.spawn_bundle(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
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

pub fn spawn_bishop(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands.spawn_bundle(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
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

pub fn spawn_rook(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands.spawn_bundle(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
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
        
pub fn spawn_pawn(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3,
) {
    commands.spawn_bundle(PbrBundle {
            transform: Transform::from_translation(position),
            ..default()
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
