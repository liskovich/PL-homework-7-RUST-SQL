use bevy::prelude::*;

// https://www.youtube.com/watch?v=j7qHwb7geIM

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (598., 676.).into(),
                title: "Rico game!".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup_system)
        .run()
}

fn setup_system(mut commands: Commands) {
    // setup camera
    commands.spawn(Camera2dBundle::default());

    // add rectangle to screen
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(150., 150.)),
            ..Default::default()
        },
        ..Default::default()
    });
}
