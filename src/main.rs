use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("SampleLevel.ldtk"),
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