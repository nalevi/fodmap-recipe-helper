use std::io;

use log::*;

mod database;
mod indigent;
mod recipes;

fn main() -> Result<(), io::Error> {
    env_logger::init();
    let result = Ok(());
    let database_conn = database::get_connection("fodmap.db3");
    database::create_db(&database_conn).unwrap();

    indigent::Indigent::from_file(
        "/Users/nalevi/RustroverProjects/fodmap-recipe-helper/tests/resources/test_indigents.csv",
    )
    .iter()
    .for_each(|indigent| {
        database::insert_indigent(&database_conn, indigent).unwrap();
    });

    let indigents_in_db = database::get_all_indigents(&database_conn).unwrap();
    indigents_in_db.iter().for_each(|indigent| {
        info!("Indigent: {:?}", indigent);
    });

    result
}
