//! Bevy plugin for 3D visualization.

use bevy::prelude::*;

/// Shared Bevy plugin for 3D visualization.
pub struct Viewer3DPlugin;

impl Plugin for Viewer3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_viewer)
            .add_systems(Update, (handle_navigation, update_visibility));
    }
}

/// Setup the 3D viewer.
fn setup_viewer(mut commands: Commands) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 100.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Directional light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });

    tracing::info!("3D viewer initialized");
}

/// Handle camera navigation.
fn handle_navigation(
    _keyboard: Res<ButtonInput<KeyCode>>,
    _mouse: Res<ButtonInput<MouseButton>>,
    mut _query: Query<&mut Transform, With<Camera3d>>,
) {
    // TODO: Implement camera controls
    // - Left-click + drag: Rotate
    // - Middle-click + drag: Pan
    // - Scroll wheel: Zoom
}

/// Update layer visibility.
fn update_visibility() {
    // TODO: Implement layer visibility toggling
}
