use amethyst::ecs::prelude::{Dispatcher, DispatcherBuilder, Resources, System};
use amethyst::core::bundle::SystemBundle;

use crate::game::components;
use crate::game::components::ui::UiComponent;

use std::time;

pub trait DispatcherSelector<Arg, Res> {
    fn select<'c>(builder: &'c mut Arg) -> &'c mut Res;
}

pub struct Base;
impl<'a, 'b> DispatcherSelector<GameData<'a, 'b>, Dispatcher<'a, 'b>> for Base {
    fn select<'c>(builder: &'c mut GameData<'a, 'b>) -> &'c mut Dispatcher<'a, 'b> {
        &mut builder.basic
    }
}
impl<'a, 'b> DispatcherSelector<GameDataBuilder<'a, 'b>, DispatcherBuilder<'a, 'b>> for Base {
    fn select<'c>(builder: &'c mut GameDataBuilder<'a, 'b>) -> &'c mut DispatcherBuilder<'a, 'b> {
        &mut builder.basic
    }
}

pub struct Adv;
impl<'a, 'b> DispatcherSelector<GameData<'a, 'b>, Dispatcher<'a, 'b>> for Adv {
    fn select<'c>(builder: &'c mut GameData<'a, 'b>) -> &'c mut Dispatcher<'a, 'b> {
        &mut builder.adv
    }
}
impl<'a, 'b> DispatcherSelector<GameDataBuilder<'a, 'b>, DispatcherBuilder<'a, 'b>> for Adv {
    fn select<'c>(builder: &'c mut GameDataBuilder<'a, 'b>) -> &'c mut DispatcherBuilder<'a, 'b> {
        &mut builder.adv
    }
}

pub struct GameData<'a, 'b> {
    basic: Dispatcher<'a, 'b>,
    adv: Dispatcher<'a, 'b>,
}

impl<'a, 'b> GameData<'a, 'b> {
    pub fn update<D: DispatcherSelector<Self, Dispatcher<'a, 'b>>>(&mut self, _: D, res: &Resources) {
        D::select(self).dispatch(res)
    }
}

#[derive(Default)]
pub struct GameDataBuilder<'a, 'b> {
    basic: DispatcherBuilder<'a, 'b>,
    adv: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> GameDataBuilder<'a, 'b> {
    pub fn with_bundle<D: DispatcherSelector<Self, DispatcherBuilder<'a, 'b>>, B: SystemBundle<'a, 'b>>(mut self, _: D, bundle: B) -> amethyst::core::bundle::Result<Self> {
        bundle.build(D::select(&mut self))?;
        Ok(self)
    }

    pub fn with<D: DispatcherSelector<Self, DispatcherBuilder<'a, 'b>>, S>(mut self, _: D, system: S, name: &str, deps: &[&str]) -> Self where for<'c> S: System<'c> + Send + 'a {
        D::select(&mut self).add(system, name, deps);
        self
    }
}

impl<'a, 'b> amethyst::DataInit<GameData<'a, 'b>> for GameDataBuilder<'a, 'b> {
    fn build(self, world: &mut amethyst::prelude::World) -> GameData<'a, 'b> {
        let pool = world.read_resource::<amethyst::core::ArcThreadPool>().clone();

        let mut basic = self.basic.with_pool(pool.clone()).build();
        let mut adv = self.adv.with_pool(pool.clone()).build();
        basic.setup(&mut world.res);
        adv.setup(&mut world.res);

        GameData {
            basic,
            adv,
        }
    }
}

#[derive(Default)]
pub struct Menu {
    ui: Option<components::ui::Menu>,
}

impl<'a, 'b> amethyst::State<GameData<'a, 'b>, amethyst::StateEvent> for Menu {
    fn on_start(&mut self, mut data: amethyst::StateData<GameData>) {
        let res = components::ui::Resources::fetch(&mut data.world);
        self.ui = Some(components::ui::Menu::new(&mut data.world, &res));
    }

    fn on_stop(&mut self, mut data: amethyst::StateData<GameData>) {
        match self.ui.take() {
            Some(ui) => ui.destroy(&mut data.world),
            None => unreach!()
        }
    }

    fn handle_event(&mut self, _data: amethyst::StateData<GameData>, event: amethyst::StateEvent) -> amethyst::Trans<GameData<'a, 'b>, amethyst::StateEvent> {
        match event {
            amethyst::StateEvent::Window(event) => if amethyst::input::is_close_requested(&event) {
                amethyst::Trans::Quit
            } else {
                amethyst::Trans::None
            },
            amethyst::StateEvent::Ui(event) => match event.event_type {
                amethyst::ui::UiEventType::ClickStop => {
                    let ui = match self.ui.as_ref() {
                        Some(ui) => ui,
                        None => unreach!()
                    };

                    //TODO: we actually get two click events?
                    info!("Click by {:?}", event.target);

                    if event.target == ui.exit_game_btn {
                        amethyst::Trans::Quit
                    } else if event.target == ui.new_game_btn {
                        amethyst::Trans::Switch(Box::new(Game::default()))
                    } else {
                        amethyst::Trans::None
                    }
                },
                _ => amethyst::Trans::None
            },
        }
    }

    fn update(&mut self, state: amethyst::StateData<GameData>) -> amethyst::Trans<GameData<'a, 'b>, amethyst::StateEvent> {
        state.data.update(Base, &state.world.res);
        amethyst::Trans::None
    }
}

const CLICK_BOUNCE_TIMEOUT: time::Duration = time::Duration::from_secs(1);

pub struct Game {
    ui: Option<components::ui::Adv>,
    last_click_inst: time::Instant,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            ui: None,
            last_click_inst: time::Instant::now(),
        }
    }
}

impl Game {
    pub fn is_close_click_bounced(&mut self) -> bool {
        let now = time::Instant::now();
        let duration = now.duration_since(self.last_click_inst);
        self.last_click_inst = now;

        CLICK_BOUNCE_TIMEOUT >= duration
    }

    pub fn ui_mut(&mut self) -> &mut components::ui::Adv {
        match self.ui.as_mut() {
            Some(ui) => ui,
            None => unreach!()
        }
    }
}

impl<'a, 'b> amethyst::State<GameData<'a, 'b>, amethyst::StateEvent> for Game {
    fn on_start(&mut self, mut data: amethyst::StateData<GameData>) {
        let res = components::ui::Resources::fetch(&mut data.world);
        self.ui = Some(components::ui::Adv::new(&mut data.world, &res));

        let sprites = components::sprites::Sprites::fetch(&mut data.world);
        sprites.demo(&mut data.world);

        let _camera = components::camera::Camera::new(&mut data.world);
    }

    fn on_stop(&mut self, mut data: amethyst::StateData<GameData>) {
        match self.ui.take() {
            Some(ui) => ui.destroy(&mut data.world),
            None => unreach!()
        }
    }

    fn handle_event(&mut self, mut data: amethyst::StateData<GameData>, event: amethyst::StateEvent) -> amethyst::Trans<GameData<'a, 'b>, amethyst::StateEvent> {
        match event {
            amethyst::StateEvent::Window(event) => if amethyst::input::is_close_requested(&event) {
                amethyst::Trans::Quit
            } else if amethyst::input::is_key_down(&event, amethyst::renderer::VirtualKeyCode::Space) {
                self.ui_mut().text.toggle_hide(&mut data.world);
                amethyst::Trans::None
            } else {
                amethyst::Trans::None
            },
            amethyst::StateEvent::Ui(event) => match event.event_type {
                amethyst::ui::UiEventType::Click => {
                    if event.target == self.ui_mut().text.close && !self.is_close_click_bounced() {
                        info!("Close text window!");
                        self.ui_mut().text.toggle_hide(&mut data.world);
                    }

                    amethyst::Trans::None
                },
                _ => amethyst::Trans::None
            }
        }
    }

    fn update(&mut self, state: amethyst::StateData<GameData>) -> amethyst::Trans<GameData<'a, 'b>, amethyst::StateEvent> {
        state.data.update(Base, &state.world.res);
        state.data.update(Adv, &state.world.res);
        amethyst::Trans::None
    }
}
