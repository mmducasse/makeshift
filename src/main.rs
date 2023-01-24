//#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_imports)]
//#![allow(non_camel_case_types)]
#![allow(unused_variables)]
//#![allow(unused_mut)]
//#![allow(unused_must_use)]

use crate::graphics::window::set_scale;

mod consts;
mod common;
mod graphics;
mod io;
mod ui;
mod level;
mod entities;
mod systems;
mod game;
mod other;
mod scene;

#[macroquad::main("Makeshift")]
async fn main() {
    println!("*** Boss Rush Jam 2023 ***");
    println!("      \"Makeshift\" ");

    set_scale(3);
    
    loop {
        let result = scene::game::run().await;
        scene::end::run(result).await;
    }
}