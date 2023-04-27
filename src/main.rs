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
pub struct Player(f32);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Structure{

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
    .insert(Player(100.0))
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
    commands.spawn(SpriteBundle{
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(1000.0, 20.0)),
            ..default()
        },
        ..default()
    }).insert(Structure {});
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>
){
    for (_, mut transform) in query.iter_mut(){
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
        // press space
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
        .register_type::<Structure>()
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_board)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_top_layer)
        .add_system(move_player)
        .add_startup_system(setup_physics)
        .run();
}
