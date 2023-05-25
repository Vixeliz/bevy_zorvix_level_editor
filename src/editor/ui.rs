use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
};
use bevy_egui::*;
use egui::Widget;

use super::plugin::{EditorImage, EditorPass};

pub fn setup(
    mut egui_user_textures: ResMut<EguiUserTextures>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    editor_pass: Res<EditorPass>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);
    egui_user_textures.add_image(image_handle.clone());
    commands.insert_resource(EditorImage(image_handle.clone()));

    // The cube that will be rendered to the texture.
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 4.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                reflectance: 0.02,
                unlit: false,
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        })
        .insert(**editor_pass);

    commands
        .spawn(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgba(1.0, 1.0, 1.0, 0.0)),
                ..default()
            },
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..default()
        })
        .insert(**editor_pass);
}

pub fn render_to_image(
    editor_image: Res<EditorImage>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut contexts: EguiContexts,
) {
    if let Some(image) = images.get_mut(&**editor_image) {
        let editor_image_id = contexts.image_id(&editor_image).unwrap();
        let ctx = contexts.ctx_mut();
        egui::SidePanel::left("Properties").show(ctx, |ui| {});

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.available_width() as u32 != 0 && ui.available_height() as u32 != 0 {
                image.resize(Extent3d {
                    width: ui.available_width() as u32,
                    height: ui.available_height() as u32,
                    ..default()
                });
            }
            ui.image(editor_image_id, ui.available_size());
        });
    }
}
