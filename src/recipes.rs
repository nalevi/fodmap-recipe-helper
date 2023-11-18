use log::{debug, error};

/// An indigent of a recipe.
#[derive(Debug, Clone)]
pub struct RecipeIndigent {
    /// The name of the indigent.
    pub name: String,

    /// The amount of the indigent in grams.
    pub amount: u32,

    /// The unique id of the indigent in the database.
    pub indigent_id: u32,
}

impl RecipeIndigent {
    fn new(name: &str, amount: u32) -> Self {
        RecipeIndigent { name: name.to_owned(), amount, indigent_id: 0 }
    }
}

impl PartialEq for RecipeIndigent {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for RecipeIndigent {}

#[derive(Debug, PartialEq, Default, Clone)]
pub enum RecipeType {
    #[default]
    Breakfast,
    Lunch,
    Dinner,
    Snack,
    Dessert,
}


#[derive(Debug)]
pub struct Recipe {
    name: String,
    indigents: Vec<RecipeIndigent>,
    steps: Vec<String>,
    recipe_id: u32,
    recipe_type: RecipeType,
}

impl Recipe {
    /// Adds an RecipeIndigent to the list of indigents of the Recipe.
    /// If the indigent is already contained by the Recipe, than update the amount field of it with the input's amount.
    ///
    /// # Examples
    ///
    /// ```
    /// let ind_corn = RecipeIndigent::new("Corn", 2);
    /// let ind_corn_again = RecipeIndigent::new("Corn", 13);
    ///
    /// let mut recipe = Recipe::new("recipe");
    ///
    /// recipe.add_indigent(ind_corn.clone());
    /// recipe.add_indigent(ind_corn_again.clone());
    ///
    /// assert_eq!(recipe.indigents.len(), 1);
    /// assert_eq!(recipe.indigents[0].amount, 15);
    /// ```
    pub(crate) fn add_indigent(&mut self, indigent: RecipeIndigent) {
        let mut indigent_already_contained = false;
        for ind in self.indigents.iter_mut() {
            if ind == &indigent {
                ind.amount += indigent.amount;
                indigent_already_contained = true;
                break;
            }
        }
        if !indigent_already_contained {
            debug!("Added indigent {} to recipe, because it was not contained.", indigent.name);
            self.indigents.push(indigent);
        }
    }

    /// Removes an RecipeIndigent from the list of indigents of the Recipe.
    /// If the indigent is not contained by the Recipe, than do nothing.
    ///
    /// # Examples
    ///
    /// ```
    /// let ind_corn = RecipeIndigent::new("Corn", 2);
    /// let ind_corn_again = RecipeIndigent::new("Corn", 13);
    /// let ind_banana = RecipeIndigent::new("Banana", 13);
    ///
    /// let mut recipe = Recipe::new("recipe");
    ///
    /// recipe.add_indigent(ind_corn.clone());
    /// recipe.add_indigent(ind_corn_again.clone());
    /// recipe.add_indigent(ind_banana.clone());
    ///
    /// assert_eq!(recipe.indigents.len(), 2);
    ///
    /// recipe.remove_indigent(ind_corn.clone());
    ///
    /// assert_eq!(recipe.indigents.len(), 1);
    /// assert_eq!(recipe.indigents[0].amount, 13);
    /// ```
    pub(crate) fn remove_indigent(&mut self, indigent: RecipeIndigent) {
        for i in 0..self.indigents.len() {
            if self.indigents[i] == indigent {
                self.indigents.remove(i);
                break;
            }
        }

        debug!("Indigent {} not found in recipe.", indigent.name);
    }

    /// Adds a step to the list of steps of the Recipe.
    /// A step is a string that describes what to do in the step.
    pub(crate) fn add_step(&mut self, step: &str) {
        self.steps.push(step.to_owned());
    }

    /// Removes a step from the list of steps of the Recipe.
    /// If the step is not contained by the Recipe, than do nothing.
    pub(crate) fn remove_step(&mut self, step_number: usize) {
        if step_number < self.steps.len() {
            self.steps.remove(step_number);
        } else {
            error!("Step number {} does not exist.", step_number);
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn indigents(&self) -> &Vec<RecipeIndigent> {
        &self.indigents
    }
    pub fn steps(&self) -> &Vec<String> {
        &self.steps
    }
    pub fn recipe_id(&self) -> u32 {
        self.recipe_id
    }

    pub fn recipe_type(&self) -> &RecipeType {
        &self.recipe_type
    }

    fn builder() -> RecipeBuilder {
        RecipeBuilder::default()
    }
}

/// A builder for the Recipe struct.
#[derive(Debug, Default)]
pub struct RecipeBuilder {
    name: String,
    indigents: Vec<RecipeIndigent>,
    steps: Vec<String>,
    recipe_id: u32,
    recipe_type: RecipeType,
}

impl RecipeBuilder {
    pub fn new(name: &str) -> RecipeBuilder {
        RecipeBuilder {
            name: name.to_owned(),
            indigents: Vec::new(),
            steps: Vec::new(),
            recipe_id: 0,
            recipe_type: RecipeType::Breakfast,
        }
    }

    pub fn recipe_type(mut self, recipe_type: &RecipeType) -> Self {
        self.recipe_type = recipe_type.clone();
        self
    }

    pub fn indigents(mut self, indigents: Vec<RecipeIndigent>) -> Self {
        self.indigents = indigents;
        self
    }

    pub fn steps(mut self, steps: Vec<String>) -> Self {
        self.steps = steps;
        self
    }

    pub fn recipe_id(mut self, recipe_id: u32) -> Self {
        self.recipe_id = recipe_id;
        self
    }

    pub fn build(self) -> Recipe {
        Recipe {
            name: self.name,
            indigents: self.indigents,
            steps: self.steps,
            recipe_id: self.recipe_id,
            recipe_type: self.recipe_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::recipes::{ RecipeBuilder, RecipeIndigent, RecipeType};

    #[test]
    fn create_indigent() {
        let ind = RecipeIndigent::new("Corn", 12);
        assert_eq!(ind.name.to_owned(), "Corn");
        assert_eq!(ind.amount, 12);
    }

    #[test]
    fn create_recipe() {
        let ind_corn = RecipeIndigent::new("Carrot", 2);
        let ind_banana = RecipeIndigent::new("Banana", 13);
        let ind_oat = RecipeIndigent::new("Oat", 250);

        let mut recipe = RecipeBuilder::new("recipe").recipe_type(&RecipeType::Lunch).build();
        recipe.add_indigent(ind_corn.clone());
        recipe.add_indigent(ind_banana.clone());
        recipe.add_indigent(ind_oat.clone());

        assert_eq!(recipe.name, "recipe");
        assert_eq!(recipe.indigents.len(), 3);
        assert!(recipe.indigents.contains(&ind_corn));
        assert_eq!(recipe.recipe_type, RecipeType::Lunch);
    }

    #[test]
    fn add_extend_indigents_of_recipe() {
        let ind_corn = RecipeIndigent::new("Corn", 2);
        let ind_corn_again = RecipeIndigent::new("Corn", 13);

        let mut recipe = RecipeBuilder::new("recipe").build();
        recipe.add_indigent(ind_corn.clone());
        assert_eq!(recipe.name, "recipe");
        assert_eq!(recipe.indigents.len(), 1);

        assert_eq!(recipe.indigents[0].amount, 2);

        recipe.add_indigent(ind_corn_again.clone());

        assert_eq!(recipe.indigents.len(), 1);
        assert_eq!(recipe.indigents[0].amount, 15);
    }

    #[test]
    fn remove_indigent_from_recipe() {
        let ind_corn = RecipeIndigent::new("Corn", 2);
        let ind_corn_again = RecipeIndigent::new("Corn", 13);
        let ind_banana = RecipeIndigent::new("Banana", 3);

        let mut recipe = RecipeBuilder::new("recipe").build();
        recipe.add_indigent(ind_corn.clone());
        recipe.add_indigent(ind_corn_again.clone());
        recipe.add_indigent(ind_banana.clone());

        assert_eq!(recipe.indigents.len(), 2);

        recipe.remove_indigent(ind_corn.clone());

        assert_eq!(recipe.indigents.len(), 1);
        assert_eq!(recipe.indigents[0].amount, 3);
        assert_eq!(recipe.indigents[0].name, "Banana");
    }

    #[test]
    fn remove_non_existent_indigent_from_recipe() {
        let ind_corn = RecipeIndigent::new("Corn", 2);
        let ind_banana = RecipeIndigent::new("Banana", 3);

        let mut recipe = RecipeBuilder::new("recipe").build();

        recipe.add_indigent(ind_banana.clone());

        assert_eq!(recipe.indigents.len(), 1);

        recipe.remove_indigent(ind_corn.clone());

        assert_eq!(recipe.indigents.len(), 1);
        assert_eq!(recipe.indigents[0].amount, 3);
        assert_eq!(recipe.indigents[0].name, "Banana");
    }

    #[test]
    fn add_step_to_recipe() {
        let mut recipe = RecipeBuilder::new("recipe").build();
        recipe.add_step("Step 1");
        recipe.add_step("Step 2");
        recipe.add_step("Step 3");

        assert_eq!(recipe.steps.len(), 3);
        assert_eq!(recipe.steps[0], "Step 1");
        assert_eq!(recipe.steps[1], "Step 2");
        assert_eq!(recipe.steps[2], "Step 3");
    }

    #[test]
    fn remove_step_from_recipe() {
        let mut recipe = RecipeBuilder::new("recipe").build();
        recipe.add_step("Step 1");
        recipe.add_step("Step 2");
        recipe.add_step("Step 3");

        assert_eq!(recipe.steps.len(), 3);
        assert_eq!(recipe.steps[0], "Step 1");
        assert_eq!(recipe.steps[1], "Step 2");
        assert_eq!(recipe.steps[2], "Step 3");

        recipe.remove_step(1);

        assert_eq!(recipe.steps.len(), 2);
        assert_eq!(recipe.steps[0], "Step 1");
        assert_eq!(recipe.steps[1], "Step 3");
    }

}