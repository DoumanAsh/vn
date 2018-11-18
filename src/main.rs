#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate amethyst;
extern crate lazy_panic;
extern crate rand;
extern crate specs;

#[macro_use]
extern crate log;

#[macro_use]
mod utils;
mod rt;
mod random;
mod game;

fn main() {
    rt::init();
    let _ = game::run();
}
