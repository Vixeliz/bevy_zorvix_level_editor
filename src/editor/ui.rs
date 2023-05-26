use bevy_infinite_grid::InfiniteGrid;
use bevy_infinite_grid::InfiniteGridBundle;
use bevy_mod_picking::prelude::RaycastPickCamera;
use bevy_mod_picking::prelude::RaycastPickTarget;
use bevy_mod_picking::PickableBundle;
use catppuccin::Flavour;
use catppuccin_egui::set_theme;
use catppuccin_egui::MOCHA;
use std::f32::consts::TAU;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_egui::*;

use super::camera::PanOrbitCamera;
use super::camera_egui::EguiWantsFocus;

pub fn setup(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    set_theme(contexts.ctx_mut(), MOCHA);
    commands
        .spawn((PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            material: materials.add(StandardMaterial {
                base_color: Color::hex(Flavour::Mocha.mauve().hex()).unwrap(),
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },))
        .insert((
            PickableBundle::default(), // Makes the entity pickable
            RaycastPickTarget::default(),
        ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 10.0, 10.0),
        ..default()
    });

    commands
        .spawn(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(
                    Color::hex(Flavour::Mocha.mantle().hex()).unwrap(),
                ),
                ..default()
            },
            ..default()
        })
        .insert(PanOrbitCamera {
            beta: TAU * 0.1,
            radius: 5.0,
            ..default()
        })
        .insert(RaycastPickCamera::default());

    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            x_axis_color: Color::hex(Flavour::Mocha.red().hex()).unwrap(),
            z_axis_color: Color::hex(Flavour::Mocha.blue().hex()).unwrap(),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn render_to_image(mut contexts: EguiContexts, mut wants_focus: ResMut<EguiWantsFocus>) {
    let ctx = contexts.ctx_mut();
    let hover_one = egui::SidePanel::left("Properties")
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
        })
        .response
        .hovered();

    let new_wants_focus = hover_one || ctx.wants_pointer_input() || ctx.wants_keyboard_input();
    let new_res = EguiWantsFocus {
        prev: wants_focus.curr,
        curr: new_wants_focus,
    };
    wants_focus.set_if_neq(new_res);
}
