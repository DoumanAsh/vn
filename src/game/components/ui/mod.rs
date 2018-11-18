use amethyst::prelude::{Builder, World};
use amethyst::assets::SimpleFormat;

mod background;
mod builder;

const FONT: &'static [u8] = include_bytes!("../../../../assets/fonts/georgia.ttf");

#[derive(Clone)]
struct AdvUi {
    pub text_background: amethyst::renderer::TextureHandle,
    pub close_background: amethyst::renderer::TextureHandle,
}

impl AdvUi {
    fn new(world: &mut World) -> Self {
        let text_background = world.read_resource::<amethyst::assets::Loader>().load_from_data(background::TEXT_WINDOW.into(), (), &world.read_resource());
        let close_background = world.read_resource::<amethyst::assets::Loader>().load_from_data(background::BLACK_BUTTON.into(), (), &world.read_resource());

        Self {
            text_background,
            close_background,
        }
    }
}

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
    adv: AdvUi,
    background: Background,
    font: amethyst::ui::FontHandle,
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

///Describes UI component interfaces
pub trait UiComponent {
    fn new(world: &mut World, resources: &Resources) -> Self;
    fn destroy(self, world: &mut World);
}

pub struct Menu {
    background: amethyst::ecs::Entity,
    pub new_game_btn: amethyst::ecs::Entity,
    pub exit_game_btn: amethyst::ecs::Entity,
}

impl UiComponent for Menu {
    fn new(world: &mut World, resources: &Resources) -> Self {
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

        let screen_dimensions = builder::get_button_size(screen_dimensions);

        let new_game_btn = builder::menu_button("btn_new_game", "Start", resources, screen_dimensions).with_position(0.0, -100.0)
                                                                                                      .build_from_world(world);
        let exit_game_btn = builder::menu_button("btn_exit_game", "Exit", resources, screen_dimensions).with_position(0.0, -225.0)
                                                                                                       .build_from_world(world);

        world.write_storage::<amethyst::ui::UiResize>().insert(new_game_btn, amethyst::ui::UiResize::new(builder::resize_button)).expect("To add UiResize");
        world.write_storage::<amethyst::ui::UiResize>().insert(exit_game_btn, amethyst::ui::UiResize::new(builder::resize_button)).expect("To add UiResize");


        Self {
            background,
            new_game_btn,
            exit_game_btn,
        }
    }

    fn destroy(self, world: &mut World) {
        let _ = world.delete_entity(self.background);
        let _ = world.delete_entity(self.new_game_btn);
        let _ = world.delete_entity(self.exit_game_btn);
    }
}

pub struct TextWindow {
    pub window: amethyst::ecs::Entity,
    pub text: amethyst::ecs::Entity,
    pub close: amethyst::ecs::Entity,
}

impl TextWindow {
    pub fn get_size(dimensions: (f32, f32)) -> (f32, f32) {
        //For width we rely on resize as of now
        (0.0, dimensions.1 / 3.84)
    }

    fn resize(transform: &mut amethyst::ui::UiTransform, dimensions: (f32, f32)) {
        let (_, height) = Self::get_size(dimensions);
        transform.height = height;
        //Approx number from empirical observation
        transform.local_y = dimensions.1 / 6.981;
    }

    ///Toggles hidden property of window.
    pub fn toggle_hide(&mut self, world: &mut World) {
        match world.write_storage::<amethyst::renderer::HiddenPropagate>().entry(self.window).expect("To get hidden component") {
            specs::storage::StorageEntry::Occupied(occupied) => {
                occupied.remove();
            },
            specs::storage::StorageEntry::Vacant(vacant) => {
                vacant.insert(amethyst::renderer::HiddenPropagate::default());
            },
        }
    }
}

pub struct Adv {
    pub text: TextWindow,
}

impl UiComponent for Adv {
    fn new(world: &mut World, resources: &Resources) -> Self {
        let text = builder::TextWindow::default().name("adv_text".to_owned())
                                                 .text("Text example")
                                                 .font(resources.font.clone(), 40.0)
                                                 .position(0.0.into(), 110.0.into(), None)
                                                 .width(0.0)
                                                 .height(200.0)
                                                 .anchor(amethyst::ui::Anchor::BottomMiddle)
                                                 .background(resources.adv.text_background.clone())
                                                 .stretch(amethyst::ui::Stretch::X { x_margin: 10.0, })
                                                 .resize(Box::new(TextWindow::resize))
                                                 .close_background(resources.adv.close_background.clone())
                                                 .build(world);

        Self {
            text
        }
    }

    fn destroy(self, world: &mut World) {
        let _ = world.delete_entity(self.text.window);
        let _ = world.delete_entity(self.text.text);
    }
}
