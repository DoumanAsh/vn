use amethyst::prelude::{Builder, World};
use amethyst::assets::SimpleFormat;

mod background {
    pub const DARK_BUTTON: [f32; 4] = [0.0, 0.0, 0.0, 0.5];
    pub const DARK_BUTTON_HOVER: [f32; 4] = [128.0, 0.0, 128.0, 0.75]; //dark purple
    pub const DARK_BUTTON_CLICK: [f32; 4] = [138.0, 0.0, 138.0, 0.95]; //dark purple
    pub const MENU_IMG: &'static [u8] = include_bytes!("../../../assets/background/menu.png");
}

const FONT: &'static [u8] = include_bytes!("../../../assets/fonts/georgia.ttf");

#[derive(Clone)]
struct Background {
    pub menu_button: amethyst::renderer::TextureHandle,
    pub menu_button_hover: amethyst::renderer::TextureHandle,
    pub menu_button_clicked: amethyst::renderer::TextureHandle,
    pub menu: amethyst::renderer::TextureHandle
}

impl Background {
    fn new(world: &mut World) -> Self {
        let menu_button = world.read_resource::<amethyst::assets::Loader>().load_from_data(background::DARK_BUTTON.into(), (), &world.read_resource());
        let menu_button_hover = world.read_resource::<amethyst::assets::Loader>().load_from_data(background::DARK_BUTTON_HOVER.into(), (), &world.read_resource());
        let menu_button_clicked = world.read_resource::<amethyst::assets::Loader>().load_from_data(background::DARK_BUTTON_CLICK.into(), (), &world.read_resource());
        let menu = amethyst::renderer::PngFormat.import(background::MENU_IMG.to_owned(), amethyst::renderer::TextureMetadata::srgb()).expect("To import builtin image");
        let menu = world.read_resource::<amethyst::assets::Loader>().load_from_data(menu, (), &world.read_resource());

        Self {
            menu_button,
            menu_button_hover,
            menu_button_clicked,
            menu,
        }
    }
}

#[derive(Clone)]
///Resources for UI
pub struct Resources {
    background: Background,
    font: amethyst::ui::FontHandle,
}

impl Resources {
    fn new(world: &mut World) -> Self {
        let font = amethyst::ui::TtfFormat.import(FONT.to_owned(), ()).expect("To import builtin font");
        let font = world.read_resource::<amethyst::assets::Loader>().load_from_data(font, (), &world.read_resource());

        Self {
            background: Background::new(world),
            font,
        }
    }

    pub fn create(world: &mut World) {
        if !world.res.has_value::<Self>() {
            let res = Self::new(world);
            world.add_resource(res);
        }
    }

    pub fn fetch(world: &mut World) -> Self {
        Self::create(world);

        world.read_resource::<Self>().clone()
    }
}

///Creates common UiButtonBuilder
fn menu_button(name: &str, text: &str, resources: &Resources, size: (f32, f32)) -> amethyst::ui::UiButtonBuilder {
    amethyst::ui::UiButtonBuilder::new(name, text).with_font(resources.font.clone())
                                                  .with_image(resources.background.menu_button.clone())
                                                  .with_hover_image(resources.background.menu_button_hover.clone())
                                                  .with_press_image(resources.background.menu_button_clicked.clone())
                                                  .with_anchor(amethyst::ui::Anchor::Middle)
                                                  .with_size(size.0, size.1)
                                                  .with_layer(5.0)
                                                  .with_font_size(20.0)
}

fn get_button_size(dimensions: (f32, f32)) -> (f32, f32) {
    (dimensions.0 * 0.2, 100.0)
}

fn resize_button(transform: &mut amethyst::ui::UiTransform, dimensions: (f32, f32)) {
    let new_dimensions = get_button_size(dimensions);
    transform.width = new_dimensions.0;
    transform.height = new_dimensions.1;
}

pub struct Menu {
    background: amethyst::ecs::Entity,
    pub new_game_btn: amethyst::ecs::Entity,
    pub exit_game_btn: amethyst::ecs::Entity,
}

impl Menu {
    pub fn new(world: &mut World, resources: &Resources) -> Self {
        let background = amethyst::ui::UiImage { texture: resources.background.menu.clone() };
        let mut overlay_transform = amethyst::ui::UiTransform::new(
            "Background".to_string(),
            amethyst::ui::Anchor::Middle,
            0.0, 0.0, 1.5,
            1.0, 1.0,
            0
        );
        overlay_transform.stretch = amethyst::ui::Stretch::XY {
            x_margin: 0.0,
            y_margin: 0.0,
        };

        let background = world.create_entity()
                              .with(overlay_transform)
                              .with(background)
                              .build();

        let screen_dimensions = {
            let screen_dimensions = world.read_resource::<amethyst::renderer::ScreenDimensions>();
            (screen_dimensions.width(), screen_dimensions.height())
        };

        let screen_dimensions = get_button_size(screen_dimensions);

        let new_game_btn = menu_button("btn_new_game", "Start", resources, screen_dimensions).with_position(0.0, -100.0)
                                                                                             .build_from_world(world);
        let exit_game_btn = menu_button("btn_exit_game", "Exit", resources, screen_dimensions).with_position(0.0, -225.0)
                                                                                              .build_from_world(world);

        world.write_storage::<amethyst::ui::UiResize>().insert(new_game_btn, amethyst::ui::UiResize::new(resize_button)).expect("To add UiResize");
        world.write_storage::<amethyst::ui::UiResize>().insert(exit_game_btn, amethyst::ui::UiResize::new(resize_button)).expect("To add UiResize");


        Self {
            background,
            new_game_btn,
            exit_game_btn,
        }

    }

    pub fn destroy(self, world: &mut World) {
        let _ = world.delete_entity(self.background);
        let _ = world.delete_entity(self.new_game_btn);
        let _ = world.delete_entity(self.exit_game_btn);
    }
}
