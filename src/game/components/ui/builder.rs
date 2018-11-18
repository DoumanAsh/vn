use amethyst::prelude::{Builder, World};

use ::utils::AssignOptionIf;

///Creates common UiButtonBuilder
pub fn menu_button(name: &str, text: &str, resources: &super::Resources, size: (f32, f32)) -> amethyst::ui::UiButtonBuilder {
    amethyst::ui::UiButtonBuilder::new(name, text).with_font(resources.font.clone())
                                                  .with_image(resources.background.menu_button.clone())
                                                  .with_hover_image(resources.background.menu_button_hover.clone())
                                                  .with_press_image(resources.background.menu_button_clicked.clone())
                                                  .with_anchor(amethyst::ui::Anchor::Middle)
                                                  .with_size(size.0, size.1)
                                                  .with_layer(5.0)
                                                  .with_font_size(20.0)
}

pub fn get_button_size(dimensions: (f32, f32)) -> (f32, f32) {
    (dimensions.0 * 0.2, 100.0)
}

pub fn resize_button(transform: &mut amethyst::ui::UiTransform, dimensions: (f32, f32)) {
    let new_dimensions = get_button_size(dimensions);
    transform.width = new_dimensions.0;
    transform.height = new_dimensions.1;
}

type ResizeFn = Box<FnMut(&mut amethyst::ui::UiTransform, (f32, f32)) + Send + Sync>;
///Text window Builder
pub struct TextWindow {
    name: Option<String>,
    //x, y, z
    position: (f32, f32, f32),
    //width, height
    dimensions: (f32, f32),
    tab_order: i32,
    anchor: amethyst::ui::Anchor,
    stretch: amethyst::ui::Stretch,
    text: Option<String>,
    text_color: [f32; 4],
    font: Option<amethyst::ui::FontHandle>,
    font_size: f32,
    image: Option<amethyst::renderer::TextureHandle>,
    resize_fn: Option<ResizeFn>,
    close_background: Option<amethyst::renderer::TextureHandle>,
}

const DEFAULT_Z: f32 = 1.0;
const DEFAULT_TXT_COLOR: [f32; 4] = [128.0, 128.0, 128.0, 1.0];

impl Default for TextWindow {
    fn default() -> Self {
        Self {
            name: None,
            position: (0.0, 0.0, DEFAULT_Z),
            dimensions: (0.0, 0.0),
            tab_order: 0,
            anchor: amethyst::ui::Anchor::TopLeft,
            stretch: amethyst::ui::Stretch::NoStretch,
            text: None,
            text_color: DEFAULT_TXT_COLOR,
            font: None,
            font_size: 32.0,
            image: None,
            resize_fn: None,
            close_background: None,
        }
    }
}

impl TextWindow {
    ///Associates identifier with window.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    ///Assigns window position
    pub fn position(mut self, x: Option<f32>, y: Option<f32>, z: Option<f32>) -> Self {
        self.position.0.assign_if(x);
        self.position.1.assign_if(y);
        self.position.2.assign_if(z);
        self
    }

    ///Assigns window's width
    pub fn width(mut self, new: f32) -> Self {
        self.dimensions.0 = new;
        self
    }

    ///Assigns window's height
    pub fn height(mut self, new: f32) -> Self {
        self.dimensions.1 = new;
        self
    }

    ///Sets Anchor for window.
    ///
    ///By default it is TopLeft
    pub fn anchor(mut self, anchor: amethyst::ui::Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    ///Set's stretch policy for element
    pub fn stretch(mut self, stretch: amethyst::ui::Stretch) -> Self {
        self.stretch = stretch;
        self
    }

    ///Sets initial text on window.
    ///
    ///By default it is empty.
    pub fn text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    #[allow(unused)]
    ///Sets text color, initial is black
    pub fn text_color(mut self, text_color: [f32; 4]) -> Self {
        self.text_color = text_color;
        self
    }

    ///Sets font.
    pub fn font(mut self, font: amethyst::ui::FontHandle, size: f32) -> Self {
        self.font = Some(font);
        self.font_size = size;
        self
    }

    ///Sets background image.
    pub fn background(mut self, image: amethyst::renderer::TextureHandle) -> Self {
        self.image = Some(image);
        self
    }

    ///Sets function that going to resize window
    ///on screen resize
    pub fn resize(mut self, new_fn: ResizeFn) -> Self {
        self.resize_fn = Some(new_fn);
        self
    }

    ///Sets texture for close Button
    pub fn close_background(mut self, new: amethyst::renderer::TextureHandle) -> Self {
        self.close_background = Some(new);
        self
    }

    ///Builds text window into Entity
    pub fn build(self, world: &mut World) -> super::TextWindow {
        let mut window_transform = amethyst::ui::UiTransform::new(
            "TextWindow".to_string(),
            self.anchor,
            self.position.0, self.position.1, self.position.2,
            self.dimensions.0, self.dimensions.1,
            self.tab_order,
        );
        window_transform.stretch = self.stretch;
        window_transform.opaque = false;

        let background = match self.image {
            Some(image) => amethyst::ui::UiImage { texture: image },
            None => panic!("Background is not set"),
        };

        let window = world.create_entity()
                          .with(window_transform)
                          .with(background)
                          .build();

        if let Some(function) = self.resize_fn {
            let resize = amethyst::ui::UiResize {
                function
            };
            world.write_storage::<amethyst::ui::UiResize>().insert(window, resize).expect("To add UiResize");
        }

        let font = self.font.expect("To have font set");

        let mut text = amethyst::ui::UiText::new(font.clone(), self.text.unwrap_or_else(|| "".to_owned()), self.text_color, self.font_size);
        text.line_mode = amethyst::ui::LineMode::Wrap;
        text.align = amethyst::ui::Anchor::TopLeft;

        let mut text_transform = amethyst::ui::UiTransform::new(
            "TextWindowText".to_string(),
            amethyst::ui::Anchor::Middle,
            0.0, 0.0, self.position.2 + 2.0,
            self.dimensions.0, self.dimensions.0,
            self.tab_order + 1,
        );
        text_transform.stretch = amethyst::ui::Stretch::XY { x_margin: 10.0, y_margin: 10.0 };
        let parent = amethyst::core::transform::components::Parent {
            entity: window,
        };

        let text = world.create_entity()
                        .with(text_transform)
                        .with(text)
                        .with(parent)
                        .build();

        let close_background = self.close_background.expect("Get Close Button background");

        let close = amethyst::ui::UiButtonBuilder::new("TextWindowClose", "X").with_font(font)
                                                                              .with_image(close_background)
                                                                              .with_anchor(amethyst::ui::Anchor::TopRight)
                                                                              .with_size(50.0, 50.0)
                                                                              .with_position(-20.0, -20.0)
                                                                              .with_font_size(30.0)
                                                                              .with_layer(self.position.2 + 200.0)
                                                                              .with_parent(window)
                                                                              .with_text_color(DEFAULT_TXT_COLOR)
                                                                              .build_from_world(world);

        super::TextWindow {
            window,
            text,
            close,
        }
    }
}
