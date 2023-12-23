use bevy::{
    core_pipeline::{
        bloom::BloomSettings, tonemapping::Tonemapping,
    },
    prelude::*,
};
use std::f32::consts::PI;

pub mod brick;
pub mod colors;
pub mod level;
pub mod materials;
pub mod menu;

#[derive(
    Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States,
)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}

/// set up a simple 3D scene
pub fn setup(mut commands: Commands) {
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            // illuminance: 1.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(20.0, 5.0, 20.0)
                .looking_at(
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::Y,
                ),
            ..default()
        },
        BloomSettings::default(),
        // CameraController::default(),
    ));
}
