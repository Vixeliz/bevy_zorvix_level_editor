use std::f32::consts::TAU;

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

use super::{
    camera::PanOrbitCamera,
    plugin::{EditorImage, EditorPass},
};

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

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..default()
            }),
            ..default()
        },
        **editor_pass,
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgba(1.0, 1.0, 1.0, 0.0)),
                ..default()
            },
            camera: Camera {
                order: 0,
                target: RenderTarget::Image(image_handle),
                ..default()
            },
            ..default()
        })
        .insert(**editor_pass)
        .insert(PanOrbitCamera {
            beta: TAU * 0.1,
            radius: 5.0,
            ..default()
        });
}

pub fn render_to_image(
    editor_image: Res<EditorImage>,
    mut images: ResMut<Assets<Image>>,
    mut contexts: EguiContexts,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
) {
    if let Some(image) = images.get_mut(&**editor_image) {
        if let Ok(mut pan_orbit) = pan_orbit_query.get_single_mut() {
            let editor_image_id = contexts.image_id(&editor_image).unwrap();
            let ctx = contexts.ctx_mut();
            egui::SidePanel::left("Properties")
                .default_width(250.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.heading("Editor");

                        ui.allocate_space(egui::Vec2::new(1.0, 100.0));

                        ui.horizontal(|ui| {
                            ui.label("Level Name: ");
                            ui.text_edit_singleline(&mut "Name");
                        });
                    });
                });

            pan_orbit.enabled = egui::CentralPanel::default()
                .show(ctx, |ui| {
                    if ui.available_width() as u32 != 0 && ui.available_height() as u32 != 0 {
                        image.resize(Extent3d {
                            width: ui.available_width() as u32,
                            height: ui.available_height() as u32,
                            ..default()
                        });
                    }
                    ui.image(editor_image_id, ui.available_size());
                })
                .response
                .hovered();
        }
    }
}
