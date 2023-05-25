use crate::prelude::*;
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

/// Level is the actual type used to serialize levels. You may notice walls and sectors store a string for textures.
/// It's up to you on how you want to use it to identifiy which texture to use. In the editor we use a hashmap.
#[derive(Component, Clone, Default)]
pub struct Level {
    // /// Controls the grid sive of the level, Lower values will affect performance.
    // pub grid_size: f32,
    pub sectors: Vec<Sector>,
    pub objects: Vec<Object>,
}

impl Level {
    pub fn generate_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]],
        );
        mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
        mesh
    }
}

#[derive(Clone, Default)]
pub struct Wall {
    pub points: [Vec3; 4],
    pub texture_name: String,
}

#[derive(Clone, Default)]
pub struct Sector {
    pub walls: Vec<Wall>,
    pub ceiling_tex_name: String,
    pub floor_tex_name: String,
}
