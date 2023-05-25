use bevy::{prelude::*, render::view::RenderLayers};
use bevy_zorvix_level_editor::prelude::Level;

use super::ui::{render_to_image, rotator_system, setup};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct EditingLevel(pub Level);

pub struct EditorPlugin;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct EditorPass(pub RenderLayers);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct EditorImage(pub Handle<Image>);

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditingLevel::default());
        app.insert_resource(EditorImage::default());
        app.insert_resource(EditorPass(RenderLayers::layer(1)));
        app.add_startup_system(setup);
        app.add_system(render_to_image);
        app.add_system(rotator_system);
    }
}
