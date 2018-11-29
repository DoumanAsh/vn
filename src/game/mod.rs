mod config;
mod graphics;
mod components;
mod state;
mod systems;

const ASSETS_DIR: &'static str = "./";

use self::state::{GameDataBuilder, Base, Adv};

pub fn run() -> amethyst::Result<()> {
    let sprite_pass = amethyst::renderer::DrawFlat2D::new().with_transparency(amethyst::renderer::ColorMask::all(), amethyst::renderer::ALPHA, None);
    //Clear screen with black
    //clear_target takes RGB colour
    let pipe = amethyst::renderer::Stage::with_backbuffer().clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                                                           //.with_pass(amethyst::renderer::DrawFlat::<amethyst::renderer::PosTex>::new())
                                                           .with_pass(sprite_pass)
                                                           .with_pass(amethyst::ui::DrawUi::new());
    let pipe = amethyst::renderer::Pipeline::build().with_stage(pipe);
    let pipe = amethyst::renderer::RenderBundle::new(pipe, Some(config::get_display()));

    let game_data = GameDataBuilder::default().with_bundle(Base, pipe.with_sprite_sheet_processor()).expect("To add bundle")
                                              .with_bundle(Base, amethyst::core::transform::bundle::TransformBundle::new()).expect("To add bundle")
                                              .with_bundle(Base, amethyst::input::InputBundle::<String, String>::new()).expect("To add bundle")
                                              .with_bundle(Base, amethyst::ui::UiBundle::<String, String>::new()).expect("To add bundle")
                                              .with(Base, amethyst::ui::UiMouseSystem::<String, String>::new(), "ui_mouse", &[])
                                              .with(Adv, systems::Demo::default(), systems::demo::NAME, &[])
                                              .with(Adv, amethyst::renderer::HideHierarchySystem::default(), "hide_hier", &[]);

    amethyst::Application::build(ASSETS_DIR, state::Menu::default()).expect("Create application builder")
                                                                    .build(game_data)
                                                                    .expect("Build application")
                                                                    .run();


    Ok(())
}
