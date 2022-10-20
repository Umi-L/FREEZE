use bevy::prelude::*;

#[derive(Component)]
struct PlayerMovement{
    speed: f32,
    physics: PhysicsObject
}

#[derive(Component)]
struct PhysicsObject{
    velocity: Vector2,
    friction: f32,
    gravity: f32,

    collider: AABB
}

struct Vector2{
    x: f32,
    y: f32
}

#[derive(Component)]
struct AABB{
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32
}

fn main() {
     App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("missing_texture.png"),
        ..default()
    }).insert(PlayerMovement{
        speed: 5.0,
        physics: PhysicsObject{
            velocity: Vector2{
                x: 0.0,
                y: 0.0
            },
            friction: 0.7,
            gravity: 0.5,

            collider: AABB{
                x1: 0.0,
                y1: 0.0,
                x2: 1.0,
                y2: 1.0,
            }
        }
    });

    commands.spawn().insert(AABB{
        x1: -100.0,
        x2: 100.0,

        y1: -1.0,
        y2: -5.0
    });
    
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut PlayerMovement)>,
){
    for (mut transform, mut player) in query.iter_mut(){
        //get inpu>ts
        if keys.pressed(KeyCode::A){
            player.physics.velocity.x += -player.speed;
        }
        if keys.pressed(KeyCode::D){
            player.physics.velocity.x += player.speed;
        }

        //apply movement
        transform.translation.x += player.physics.velocity.x;
        transform.translation.y += player.physics.velocity.y;

        //apply friction
        player.physics.velocity.x *= player.physics.friction; 

        //apply gravity
        player.physics.velocity.y -= player.physics.gravity;
    }
}

//applies forces to a physics object
fn apply_forces(
    force: Vector2, 
    object: &mut PhysicsObject,
    mut query: Query<&AABB>
){
    //itterate thru all bounding boxes
    for mut bounds in query.iter() {

        if bounds == object.collider{
            continue;
        }

        if aabb_collision(bounds, object.collider){
            
        }
    }
}

fn aabb_collision(box1: AABB, box2: AABB) -> bool {
    false
}
