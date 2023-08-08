use bevy::prelude::*;
#[cfg(feature = "debug")]
use bevy_editor_pls::prelude::*;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "debug")]
    app.add_plugins((EditorPlugin::default()));

    app.add_systems(Update, bevy::window::close_on_esc);
    app.run();
}
