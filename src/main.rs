use bevy::{prelude::*, window::WindowResolution, transform::commands, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Field{}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Player {
    shooting_cooldown: Timer,
    bullet_offset: Vec2,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Bullet{
    velocity: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime{
    timer: Timer,
}

fn setup_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle{
        
        ..default()
    });
}

fn spawn_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>){
    commands.spawn((SpriteBundle{
        sprite: Sprite {
            color: Color::SALMON,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        ..default()
    },
    RigidBody::Dynamic,
    Velocity::zero(),
    Collider::cuboid(20.0, 20.0)
    ))
    .insert(Name::new("Player"))
    .insert(Player{shooting_cooldown: Timer::from_seconds(0.5, TimerMode::Repeating), bullet_offset: Vec2::ZERO})
    .insert( Transform::from_translation(Vec3::new(-590.0, 0.0, 1.0)));

}

fn spawn_top_layer(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
){
   commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-590., 0., 2.)),
        ..default()
    });
}

fn setup_board(mut commands: Commands ){
        commands.spawn(SpriteBundle{
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                ..default()
            }, 
            ..default()
        })
        .insert(Field {})
        .insert(Name::new("Brett"));
    //spawn a plattform which is the ground
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(2000.0, 100.0)),
            ..default()
        },
        ..default()
    })
    .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

}

fn move_player(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Player, &mut Transform)>,
    timer: Res<Time>,
    ){
    for (player_entity,mut player, mut transform) in query.iter_mut(){
        if keyboard_input.pressed(KeyCode::W){
            transform.translation.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S){
            transform.translation.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A){
            transform.translation.x -= 5.0;
        }
        if keyboard_input.pressed(KeyCode::D){
            transform.translation.x += 5.0;
        }
        //if the player presses space, the player shoots/spawns a bullet
        
        if keyboard_input.pressed(KeyCode::Space){
            //also check if cooldown is over to spawn a bullet
            player.shooting_cooldown.tick(timer.delta());
            if player.shooting_cooldown.finished(){
                commands.spawn((SpriteBundle{
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..default()
                    },
                    ..default()
                },
                //spawn bullets which dont collide with the player
                
                Velocity::zero(),
                Collider::cuboid(2.0, 2.0)
            ))
            .insert(TransformBundle::from(Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0)))
            .insert(Name::new("Bullet"))
            .insert(Lifetime {timer: Timer::from_seconds(1.0, TimerMode::Once)})
            .insert(Bullet{ velocity: 8.0});
            }  
        }
    }
}
//spawn a bullet and move it in the direction the player was moving when spawned

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time : Res<Time>
    ){
    for(entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished(){
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn move_bullets(
    mut commands: Commands,
    mut bullets: Query<(&mut Bullet, &mut Lifetime, &mut Transform)>
){
    for(mut bullet, mut time, mut transform) in &mut bullets {
        //move the bullet in the direction depending on the key pressed
        transform.translation.x += bullet.velocity;

    }
}

fn setup_physics(mut commands: Commands){
        commands
        .spawn(Collider::cuboid(1000.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(20.0, 20.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
    commands
        .spawn(Collider::cuboid(10.0, 60.0))
        .insert(TransformBundle::from(Transform::from_xyz(-200.0, -100.0, 0.0)))
        .insert(Name::new("Mauer"));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NAVY))
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
            title: "AWESOMER".into(),
            resolution: WindowResolution::new(WIDTH, HEIGHT),
            ..default()
            }),
            ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .register_type::<Player>()
        .register_type::<Field>()
        .register_type::<Bullet>()
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_board)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_top_layer)
        .add_startup_system(setup_physics)
        .add_system(move_player)
        .add_system(move_bullets)
        .add_system(bullet_despawn)
        .run();
}
