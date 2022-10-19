use bevy::prelude::*;

#[derive(Component)]
struct PlayerMovement{
    speed: f32,
    velocity: Vector2
}

struct Vector2{
    x: f32,
    y: f32
}

fn main() {
     App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("missing_texture.png"),
        ..default()
    });
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut PlayerMovement)>,
){
    for (mut transform, mut player) in query.iter_mut(){
        //get inpu>ts
        if keys.pressed(KeyCode::A){
            player.velocity.x += -player.speed;
        }
        if keys.pressed(KeyCode::D){
            player.velocity.x += player.speed;
        }


        //apply movement
        transform.translation.x += player.velocity.x;
        transform.translation.y += player.velocity.y;
    }
}
