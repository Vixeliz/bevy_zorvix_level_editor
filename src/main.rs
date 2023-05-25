pub mod editor;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use editor::plugin::EditorPlugin;

fn main() {
    // Create app
    let mut app = App::new();

    // Add plugins
    app.add_plugins(DefaultPlugins);
    app.add_plugin(EguiPlugin);
    app.add_plugin(EditorPlugin);

    // Run app
    app.run();
}
