//#![forbid(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_imports)]
//#![allow(non_camel_case_types)]
#![allow(unused_variables)]
//#![allow(unused_mut)]
//#![allow(unused_must_use)]

use crate::{
    graphics::{window::set_scale, buffer::buffer_mut}, 
    consts::SCREEN_SIZE
};

mod consts;
mod common;
mod graphics;
mod io;
mod game;
mod test;

#[macroquad::main("Makeshift")]
async fn main() {
    println!("*** Boss Rush Jam 2023 ***");
    println!("      \"Makeshift\" ");

    set_scale(2);
    
    buffer_mut().init(SCREEN_SIZE);

    test::game::run().await;
    //game::run().await;
}