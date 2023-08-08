use animate::AnimatePlugin;
use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_editor_pls::prelude::*;
use player::PlayerPlugin;

mod animate;
mod player;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::GRAY))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));

    #[cfg(feature = "debug")]
    app.add_plugins(EditorPlugin::default());

    app.add_plugins((AnimatePlugin, PlayerPlugin))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, bevy::window::close_on_esc);
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
