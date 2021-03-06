#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
