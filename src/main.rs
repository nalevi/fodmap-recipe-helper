use std::io;

use log::*;

use tui::backend::Backend;
use crate::menu::draw_menu;

mod recipes;
mod indigent;
mod database;
mod menu;

fn main() -> Result<(), io::Error> {
    env_logger::init();
    info!("Welcome to the Recipe Generator!");

    let mut database = database::RecipeDatabase::new();
    draw_menu(&mut database);

    info!("=== Bye Bye ===");


    Ok(())
}
