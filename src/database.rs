use std::collections::HashMap;
use std::borrow::Borrow;

use crate::recipes::{Recipe, RecipeBuilder};
use crate::indigent::Indigent;

// static mut INDIGENT_ID: Arc<u32> = Arc::new(0);
// static mut RECIPE_ID: Arc<u32> = Arc::new(0);

/// An in-database for recipes and indigents.
/// This struct is used to store recipes and indigents.
pub struct RecipeDatabase {
    recipes: HashMap<u32, Recipe>,
    indigents: HashMap<u32, Indigent>,
}

impl RecipeDatabase {
    /// Create a new RecipeDatabase.
    pub fn new() -> RecipeDatabase {
        RecipeDatabase {
            recipes: HashMap::new(),
            indigents: HashMap::new(),
        }
    }

    /// Add a recipe to the database.
    pub fn add_recipe(&mut self, recipe: Recipe) {
        let id = 1 + self.recipes.len() as u32;

        let recipe_to_insert = RecipeBuilder::new(recipe.name())
            .recipe_type(recipe.recipe_type().clone().borrow())
            .indigents(recipe.indigents().clone())
            .steps(recipe.steps().clone())
            .recipe_id(id)
            .build();

        self.recipes.insert(recipe_to_insert.recipe_id(), recipe_to_insert);
    }

    /// Add an indigent to the database.
    pub fn add_indigent(&mut self, indigent: Indigent) {
        self.indigents.insert(indigent.indigent_id(), indigent);
    }

    /// Get a recipe from the database.
    pub fn get_recipe_by_id(&self, recipe_id: u32) -> Option<&Recipe> {
        self.recipes.get(&recipe_id)
    }

    pub fn get_recipe_by_name(&self, recipe_name: &str) -> Option<&Recipe> {
        for recipe in self.recipes.values() {
            if recipe.name() == recipe_name {
                return Some(recipe);
            }
        }
        None
    }

    /// Get an indigent from the database.
    pub fn get_indigent(&self, indigent_id: u32) -> Option<&Indigent> {
        self.indigents.get(&indigent_id)
    }

    /// Get all recipes from the database.
    pub fn get_all_recipes(&self) -> Vec<&Recipe> {
        self.recipes.values().collect()
    }

    /// Get all indigents from the database.
    pub fn get_all_indigents(&self) -> Vec<&Indigent> {
        self.indigents.values().collect()
    }

    /// Remove a recipe from the database.
    pub fn remove_recipe(&mut self, recipe_id: u32) -> Option<Recipe> {
        self.recipes.remove(&recipe_id)
    }

    /// Remove an indigent from the database.
    pub fn remove_indigent(&mut self, indigent_id: u32) -> Option<Indigent> {
        self.indigents.remove(&indigent_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::recipes::{RecipeBuilder};
    use crate::indigent::{Indigent, IndigentType};
    use crate::database::RecipeDatabase;


    #[test]
    fn create_database() {
        let database = RecipeDatabase::new();
        assert_eq!(database.recipes.len(), 0);
        assert_eq!(database.indigents.len(), 0);
    }

    #[test]
    fn add_recipe() {
        let mut database = RecipeDatabase::new();
        let recipe = RecipeBuilder::new("recipe").build();
        database.add_recipe(recipe);
        assert_eq!(database.recipes.len(), 1);
        assert_eq!(database.recipes.get(&1).unwrap().name(), "recipe");
        assert_eq!(database.recipes.get(&1).unwrap().recipe_id(), 1);
    }

    #[test]
    fn add_indigent() {
        let mut database = RecipeDatabase::new();
        let indigent = Indigent::new("indigent", IndigentType::Other);
        database.add_indigent(indigent);
        assert_eq!(database.indigents.len(), 1);
    }

    #[test]
    fn get_recipe_by_id() {
        let mut database = RecipeDatabase::new();
        let recipe = RecipeBuilder::new("recipe").build();
        database.add_recipe(recipe);
        let recipe = database.get_recipe_by_id(1).unwrap();
        assert_eq!(recipe.name(), "recipe");
    }

    #[test]
    fn get_indigent() {
        let mut database = RecipeDatabase::new();
        let indigent = Indigent::new("indigent", IndigentType::Other);
        database.add_indigent(indigent);
        let indigent = database.get_indigent(0).unwrap();
        assert_eq!(indigent.name, "indigent");
    }

    #[test]
    fn get_all_recipes() {
        let mut database = RecipeDatabase::new();

        let recipe = RecipeBuilder::new("recipe").build();
        database.add_recipe(recipe);

        let recipe = RecipeBuilder::new("recipe2").build();
        database.add_recipe(recipe);

        let recipes = database.get_all_recipes();
        assert_eq!(recipes.len(), 2);
    }
}