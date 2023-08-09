use animate::AnimatePlugin;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_editor_pls::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use world_build::WorldBuildPlugin;

mod state;
mod animate;
mod player;
mod world_build;
mod camera;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::GRAY))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    #[cfg(feature = "debug")]
    app.add_plugins(EditorPlugin::default());

    app.add_plugins((AnimatePlugin, PlayerPlugin, WorldBuildPlugin, CameraPlugin))
        .add_systems(Update, bevy::window::close_on_esc);
    app.run();
}
