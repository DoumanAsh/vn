#![allow(unused)]

use amethyst::core::nalgebra::Matrix;

/// Converts a vector of vertices into a mesh.
pub fn create_mesh(world: &amethyst::prelude::World, vertices: Vec<amethyst::renderer::PosTex>) -> amethyst::renderer::MeshHandle {
    let loader = world.read_resource::<amethyst::assets::Loader>();
    loader.load_from_data(vertices.into(), (), &world.read_resource())
}

/// Creates a solid material of the specified colour.
pub fn create_colour_material(world: &amethyst::prelude::World, colour: [f32; 4]) -> amethyst::renderer::Material {
    let mat_defaults = world.read_resource::<amethyst::renderer::MaterialDefaults>();
    let loader = world.read_resource::<amethyst::assets::Loader>();

    let albedo = loader.load_from_data(colour.into(), (), &world.read_resource());

    amethyst::renderer::Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}

/// Generates six vertices forming a rectangle.
pub fn generate_rectangle_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<amethyst::renderer::PosTex> {
    vec![
        amethyst::renderer::PosTex {
            position: [left, bottom, 0.0].into(),
            tex_coord: [0.0, 0.0].into(),
        },
        amethyst::renderer::PosTex {
            position: [right, bottom, 0.0].into(),
            tex_coord: [1.0, 0.0].into(),
        },
        amethyst::renderer::PosTex {
            position: [left, top, 0.0].into(),
            tex_coord: [1.0, 1.0].into(),
        },
        amethyst::renderer::PosTex {
            position: [right, top, 0.0].into(),
            tex_coord: [1.0, 1.0].into(),
        },
        amethyst::renderer::PosTex {
            position: [left, top, 0.0].into(),
            tex_coord: [0.0, 1.0].into(),
        },
        amethyst::renderer::PosTex {
            position: [right, bottom, 0.0].into(),
            tex_coord: [0.0, 0.0].into(),
        },
    ]
}
