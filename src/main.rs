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
        .add_systems(Update, (
            // use inbuild bevy system to close game on esc
            bevy::window::close_on_esc,
            player_movement
        ))
        .run();
}

// Setup Game world and entities
fn setup(
    mut commands: Commands
) {
    // Create player camera
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Bird {
    pub jumping: bool,
    pub velocity: f32,
}

// creates player character
fn spawn_bird(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
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
        },
        Bird {
            jumping: false,
            velocity: 0.,
        }
    ));
} 

fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Bird, &mut Transform)>,
    time: Res<Time>
) {
    for (mut bird, mut transform) in query.iter_mut() {
        if input.just_pressed(KeyCode::Space) {
            bird.jumping = true;
        }
        
        if bird.jumping {
            bird.velocity = 1000.;
            bird.jumping = false;
        }

        transform.translation.y += bird.velocity * time.delta_seconds();

        bird.velocity -= 35.;

        // cap velocity so u dont go faster then the speed of sound!
        if bird.velocity < -1000. {
            bird.velocity = -1000.;
        }
    }    
}
