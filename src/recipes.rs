/// An indigent of a recipe.
#[derive(Debug, Clone)]
struct RecipeIndigent {
    /// The name of the indigent.
    name: String,

    /// The amount of the indigent in grams.
    amount: u32,
}

impl RecipeIndigent {
    fn new(name: &str, amount: u32) -> Self {
        RecipeIndigent { name: name.to_owned(), amount }
    }
}

impl PartialEq for RecipeIndigent {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for RecipeIndigent {}

#[derive(Debug)]
pub(crate) struct Recipe {
    name: String,
    indigents: Vec<RecipeIndigent>,

}

impl Recipe {
    /// Creates a new Recipe with the name coming from the input string.
    pub(crate) fn new(name: &str) -> Self {
        Recipe {
            name: name.to_owned(),
            indigents: Vec::new(),
        }
    }

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
            self.indigents.push(indigent);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::recipes::{Recipe, RecipeIndigent};

    #[test]
    fn create_inigent() {
        let ind = RecipeIndigent::new("Corn", 12);
        assert_eq!(ind.name.to_owned(), "Corn");
        assert_eq!(ind.amount, 12);
    }

    #[test]
    fn create_recipe() {
        let ind_corn = RecipeIndigent::new("Carrot", 2);
        let ind_banana = RecipeIndigent::new("Banana", 13);
        let ind_oat = RecipeIndigent::new("Oat", 250);

        let mut recipe = Recipe::new("recipe");
        recipe.add_indigent(ind_corn.clone());
        recipe.add_indigent(ind_banana.clone());
        recipe.add_indigent(ind_oat.clone());

        assert_eq!(recipe.name, "recipe");
        assert_eq!(recipe.indigents.len(), 3);
        assert!(recipe.indigents.contains(&ind_corn));

    }

    # [test]
    fn add_extend_indigents_of_recipe() {
        let ind_corn = RecipeIndigent::new("Corn", 2);
        let ind_corn_again = RecipeIndigent::new("Corn", 13);

        let mut recipe = Recipe::new("recipe");
        recipe.add_indigent(ind_corn.clone());
        assert_eq!(recipe.name, "recipe");
        assert_eq!(recipe.indigents.len(), 1);

        assert_eq!(recipe.indigents[0].amount, 2);

        recipe.add_indigent(ind_corn_again.clone());

        assert_eq!(recipe.indigents.len(), 1);
        assert_eq!(recipe.indigents[0].amount, 15);
    }
}