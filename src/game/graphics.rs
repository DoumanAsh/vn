#![allow(unused)]

use amethyst::prelude::{World};
use amethyst::core::nalgebra::Matrix;

use std::path;

pub trait TextureLoader {
    fn load_file<F: amethyst::assets::Format<amethyst::renderer::Texture>>(path: &str, format: F, world: &World) -> amethyst::renderer::TextureHandle {
        let loader = world.read_resource::<amethyst::assets::Loader>();
        loader.load(path, amethyst::renderer::PngFormat, amethyst::renderer::TextureMetadata::srgb(), (), &world.read_resource())
    }

    fn load_sprite_sheet(path: &str, texture: amethyst::renderer::TextureHandle, world: &World) -> amethyst::renderer::SpriteSheetHandle {
        let mut path_buf = path::Path::new(path).to_path_buf();
        path_buf.set_extension("ron");
        let path = match path_buf.to_str() {
            Some(path) => path,
            None => unreach!(),
        };

        let loader = world.read_resource::<amethyst::assets::Loader>();
        loader.load(path, amethyst::renderer::SpriteSheetFormat, texture, (), &world.read_resource())
    }

    fn load(self, world: &World) -> amethyst::renderer::SpriteSheetHandle;
}

pub enum Sprite {
    Path(&'static str)
}

impl TextureLoader for Sprite {
    fn load(self, world: &World) -> amethyst::renderer::SpriteSheetHandle {
        match self {
            Sprite::Path(path) => {
                let texture = if path.ends_with(".png") {
                    Self::load_file(path, amethyst::renderer::PngFormat, world)
                } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
                    Self::load_file(path, amethyst::renderer::JpgFormat, world)
                } else {
                    panic!("Unknown sprite format")
                };

                Self::load_sprite_sheet(path, texture, world)
            }
        }
    }
}

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
