use log::*;

mod recipes;
mod indigent;
mod database;


fn main() {
    env_logger::init();
    info!("Welcome to the Recipe Generator!");

    let mut database = database::RecipeDatabase::new();
    let indigent_carrot = indigent::Indigent::new("Carrot", indigent::IndigentType::Vegetable);
    let indigent_chicken_breast = indigent::Indigent::new("Chicken Breast", indigent::IndigentType::Meat);

    let recipe1 = recipes::RecipeBuilder::new("recipe with carrot").recipe_type(&recipes::RecipeType::Lunch).build();
    let recipe2 = recipes::RecipeBuilder::new("recipe with chicken breast").recipe_type(&recipes::RecipeType::Dinner).build();

    database.add_indigent(indigent_carrot);
    database.add_indigent(indigent_chicken_breast);

    database.add_recipe(recipe1);
    database.add_recipe(recipe2);

    let recipes = database.get_all_recipes();
    for recipe in recipes {
        error!("Recipe: {}", recipe.name());
        error!("Indigents:");
        for indigent in recipe.indigents() {
            error!("  - {} ({})", indigent.name, indigent.amount);
        }
        error!("Steps:");
        for step in recipe.steps() {
            error!("  - {}", step);
        }
        error!("Type: {:?}", recipe.recipe_type());

    }

    error!("=== Bye Bye ===");
}
