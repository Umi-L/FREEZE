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
    x: f32,
    y: f32,
    
    width: f32,
    height: f32
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
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0
            }
        }
    });

    commands.spawn().insert(AABB{
        x: -100.0,
        y: -1.0,
        
        width: 100.0,
        height: 1.0
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

        //apply friction
        player.physics.velocity.x *= player.physics.friction; 

        //apply gravity
        player.physics.velocity.y -= player.physics.gravity;

        //move player
        apply_forces(&mut player.physics);
    }
}

//applies forces to a physics object
fn apply_forces(
    object: &mut PhysicsObject,
    query: Query<&AABB>
){
    //velocity is the ammount moved per frame, but we want to check steps instead of moving all at once to have a cleaner collision.

    //get the amount of steps needed to move the object
    let steps: i32 = object.velocity.y.floor() as i32;

    for _ in 0..steps {

        //itterate thru all bounding boxes
        for bounds in query.iter() {

            //if references are the same, skip
            if bounds as *const _ == &object.collider as *const _ {
                continue;
            }

            if aabb_collision(bounds, &object.collider){
                object.velocity.y = 0.0;
            }
        }
    }
}

fn aabb_collision(r1: &AABB, r2: &AABB) -> bool {
    return r1.x <= (r2.x + r2.width) && (r1.x + r1.width) >= r2.x && r1.y <= (r2.y + r2.height) && (r1.y + r1.height) >= r2.y;
}
