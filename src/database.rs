use log::{error, info};
use sqlite::Error;
use std::borrow::Borrow;
use std::collections::HashMap;

use crate::indigent::{Indigent, IndigentType};
use crate::recipes::{Recipe, RecipeBuilder};

pub fn get_connection() -> sqlite::Connection {
    let connection = sqlite::open("fodmapdb").unwrap();

    connection
}

pub fn create_db(conn: &sqlite::Connection) -> Result<(), sqlite::Error> {
    let query = "DROP TABLE IF EXISTS indigents";
    let mut stmt = conn.prepare(query)?;

    if let Err(e) = stmt.next() {
        error!("Error dropping table indigents: {}", e);
    }

    let query = "CREATE TABLE indigents (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        food_type TEXT NOT NULL
    )";
    let mut stmt = conn.prepare(query)?;

    if let Err(e) = stmt.next() {
        error!("Error creating table indigents: {}", e);
    }

    let query = "DROP TABLE IF EXISTS recipes";
    let mut stmt = conn.prepare(query)?;

    if let Err(e) = stmt.next() {
        error!("Error dropping table recipes: {}", e);
    }

    let query = "CREATE TABLE recipes (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        ingredients TEXT NOT NULL,
        instructions TEXT NOT NULL
    )";
    let mut stmt = conn.prepare(query)?;

    if let Err(e) = stmt.next() {
        error!("Error creating table recipes: {}", e);
    }

    Ok(())
}

pub fn get_indigent_by_id(conn: &sqlite::Connection, id: u32) -> Result<Indigent, sqlite::Error> {
    let query = "SELECT * FROM indigents WHERE id = ?";
    let mut stmt = conn.prepare(query)?;

    stmt.bind((1, id as i64))?;

    let mut indigent = Indigent::new("", 0, IndigentType::Other);
    while let sqlite::State::Row = stmt.next()? {
        let ind_t = stmt.read::<i64, usize>(2)? as u32;
        indigent = Indigent::new(
            &stmt.read::<String, usize>(1)?,
            stmt.read::<i64, usize>(0)? as u32,
            IndigentType::from_u32(ind_t),
        );
    }

    Ok(indigent)
}

pub fn get_all_indigents(conn: &sqlite::Connection) -> Result<Vec<Indigent>, sqlite::Error> {
    let query = "SELECT * FROM indigents";
    let mut stmt = conn.prepare(query)?;

    let mut indigents = Vec::new();
    while let sqlite::State::Row = stmt.next()? {
        let ind_t = stmt.read::<i64, usize>(2)? as u32;
        indigents.push(Indigent::new(
            &stmt.read::<String, usize>(1)?,
            stmt.read::<i64, usize>(0)? as u32,
            IndigentType::from_u32(ind_t),
        ));
    }

    Ok(indigents)
}

pub fn insert_indigent(conn: &sqlite::Connection, indigent: &Indigent) -> Result<(), sqlite::Error> {
    let query = "INSERT INTO indigents (name, food_type) VALUES (?, ?)";
    let mut stmt = conn.prepare(query)?;

    stmt.bind((1, indigent.name.as_str()))?;
    stmt.bind::<(usize, i64)>((2, indigent.indigent_type.to_u32().into()))?;

    if let Err(e) = stmt.next() {
        error!("Error inserting indigent: {}", e);
    }

    Ok(())
}

pub  fn delete_indigent(conn: &sqlite::Connection, indigent: &Indigent) -> Result<(), sqlite::Error> {
    let query = "DELETE FROM indigents WHERE id = ?";
    let mut stmt = conn.prepare(query)?;

    stmt.bind((1, indigent.indigent_id() as i64))?;

    if let Err(e) = stmt.next() {
        error!("Error deleting indigent: {}", e);
    }

    Ok(())
}

pub fn update_indigent(conn: &sqlite::Connection, indigent: &Indigent) -> Result<(), sqlite::Error> {
    let query = "UPDATE indigents SET name = ?, food_type = ? WHERE id = ?";
    let mut stmt = conn.prepare(query)?;

    stmt.bind((1, indigent.name.as_str()))?;
    stmt.bind::<(usize, i64)>((2, indigent.indigent_type.to_u32().into()))?;
    stmt.bind((3, indigent.indigent_id() as i64))?;

    if let Err(e) = stmt.next() {
        error!("Error updating indigent: {}", e);
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
