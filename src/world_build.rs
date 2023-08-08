use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::LdtkIntCellAppExt, LdtkPlugin, LdtkWorldBundle, LevelSelection};

use self::bundles::{DirtRoadBundle, TerraceBundle};

mod bundles;
mod components;
mod systems;

pub struct WorldBuildPlugin;

impl Plugin for WorldBuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_systems(Startup, load_ldtk)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell::<DirtRoadBundle>(1)
            .register_ldtk_int_cell::<TerraceBundle>(2);
    }
}

fn load_ldtk(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("bevy-rpg-try.ldtk"),
        ..default()
    });
}
