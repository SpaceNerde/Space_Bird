use bevy::prelude::*;
use bevy::window::WindowMode::BorderlessFullscreen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Bird".to_string(),
                    mode: BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }))
        .add_systems(Startup, (setup, spawn_bird))
        // use inbuild bevy system to close game on esc
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

// Setup Game world and entities
fn setup(
    mut commands: Commands
) {
    // Create player camera
    commands.spawn(Camera2dBundle::default());
}

// creates player character
fn spawn_bird(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            texture: asset_server.load("bird.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(125., 125.)),
                ..default()
            },
            ..default()
        }
    );
}   
