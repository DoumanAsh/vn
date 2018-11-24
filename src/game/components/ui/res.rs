use amethyst::prelude::{World};
use amethyst::assets::SimpleFormat;

pub const DARK_BUTTON: [f32; 4] = [0.0, 0.0, 0.0, 0.5];
pub const DARK_BUTTON_HOVER: [f32; 4] = [128.0, 0.0, 128.0, 0.75]; //dark purple
pub const DARK_BUTTON_CLICK: [f32; 4] = [138.0, 0.0, 138.0, 0.95]; //dark purple
pub const MENU_IMG: &'static [u8] = include_bytes!("../../../../assets/background/menu.png");

pub const BLACK_BUTTON: [f32; 4] = [128.0, 128.0, 128.0, 1.0];
pub const TEXT_WINDOW: [f32; 4] = [128.0, 0.0, 128.0, 0.35]; //dark purple
const FONT: &'static [u8] = include_bytes!("../../../../assets/fonts/georgia.ttf");

#[derive(Clone)]
pub struct AdvUi {
    pub text_background: amethyst::renderer::TextureHandle,
    pub close_background: amethyst::renderer::TextureHandle,
}

impl AdvUi {
    fn new(world: &mut World) -> Self {
        let text_background = world.read_resource::<amethyst::assets::Loader>().load_from_data(TEXT_WINDOW.into(), (), &world.read_resource());
        let close_background = world.read_resource::<amethyst::assets::Loader>().load_from_data(BLACK_BUTTON.into(), (), &world.read_resource());

        Self {
            text_background,
            close_background,
        }
    }
}

#[derive(Clone)]
pub struct Background {
    pub menu_button: amethyst::renderer::TextureHandle,
    pub menu_button_hover: amethyst::renderer::TextureHandle,
    pub menu_button_clicked: amethyst::renderer::TextureHandle,
    pub menu: amethyst::renderer::TextureHandle
}

impl Background {
    fn new(world: &mut World) -> Self {
        let menu_button = world.read_resource::<amethyst::assets::Loader>().load_from_data(DARK_BUTTON.into(), (), &world.read_resource());
        let menu_button_hover = world.read_resource::<amethyst::assets::Loader>().load_from_data(DARK_BUTTON_HOVER.into(), (), &world.read_resource());
        let menu_button_clicked = world.read_resource::<amethyst::assets::Loader>().load_from_data(DARK_BUTTON_CLICK.into(), (), &world.read_resource());
        let menu = amethyst::renderer::PngFormat.import(MENU_IMG.to_owned(), amethyst::renderer::TextureMetadata::srgb()).expect("To import builtin image");
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
    pub adv: AdvUi,
    pub background: Background,
    pub font: amethyst::ui::FontHandle,
}

impl Resources {
    fn new(world: &mut World) -> Self {
        let font = amethyst::ui::TtfFormat.import(FONT.to_owned(), ()).expect("To import builtin font");
        let font = world.read_resource::<amethyst::assets::Loader>().load_from_data(font, (), &world.read_resource());

        Self {
            adv: AdvUi::new(world),
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
