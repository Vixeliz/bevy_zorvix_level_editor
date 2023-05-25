use bevy::prelude::*;

/// Object is a type to represent any object that isn't part of the level geometry itself. Ie a tree or barrel.
/// If you need things like being able to interact you should use bevys ecs to help with that.
#[derive(Component, Default, Clone)]
pub struct Object {
    pub object_name: String,
    pub mesh_name: String,
    pub position: Vec3,
}
