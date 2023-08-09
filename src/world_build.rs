use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::LdtkIntCellAppExt, EntityInstance, IntGridCell, LdtkPlugin, LdtkWorldBundle,
    LevelSelection,
};

use self::bundles::TerraceBundle;

mod bundles;
mod components;
mod systems;

pub struct WorldBuildPlugin;

impl Plugin for WorldBuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_systems(Startup, load_ldtk)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell_for_layer::<TerraceBundle>("Place", 2);

        #[cfg(feature = "debug")]
        app.register_type::<EntityInstance>()
            .register_type::<IntGridCell>();
    }
}

fn load_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("bevy-rpg-try.ldtk"),
        ..default()
    });
}
