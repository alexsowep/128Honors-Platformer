use std::process::CommandArgs;
use bevy_rapier2d::prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_ecs_ldtk::prelude::*;

mod components;
mod systems;

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        title: "Alejandro Platformer Game".to_string(),
        mode: bevy::window::WindowMode::Windowed,
        width: 500.,
        height: 300.,
        ..Default::default()
    })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.0),
            ..Default::default()
        })
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .add_system(systems::camera_fit_inside_current_level)
        .add_system(systems::spawn_wall_collision)
        .add_system(systems::movement)
        .add_system(systems::ground_detection)
        .add_system(systems::spawn_ground_sensor)
        .add_system(systems::update_on_ground)
        .add_system(systems::laser_system)
        .add_system(systems::apply_velocity)
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .register_ldtk_int_cell::<components::WallBundle>(1)
        .register_ldtk_int_cell::<components::WallBundle>(3)
        .register_ldtk_entity::<components::PlayerBundle>("Player")
        .register_ldtk_entity::<components::MobBundle>("Mob")
        .run();
}


    // Note the backgrounds are associated with a camera.
    


fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(Camera2dBundle::default());
    
     //commands.spawn_bundle(SpriteBundle {
     //   texture: asset_server.load("Background.png"),
        //transform: Transform::from_scale(Vec3::new(1.5, 1.5, 0.0)),
     //   ..Default::default() 
  //  });
    
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("FirstLevel.ldtk"),
        ..Default::default()
    });
}


#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    //a: ComponentA,
    //b: ComponentB,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}