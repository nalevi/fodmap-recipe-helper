
#[derive(Debug, Clone, PartialEq)]
pub enum IndigentType {
    Vegetable,
    Fruit,
    Seed,
    Dairy,
    Meat,
    Fish,
    Other,
}

impl IndigentType {
    pub fn from_u32(input: u32) -> IndigentType {
        match input {
            1 => IndigentType::Vegetable,
            2 => IndigentType::Fruit,
            3 => IndigentType::Seed,
            4 => IndigentType::Dairy,
            5 => IndigentType::Meat,
            6 => IndigentType::Fish,
            _ => IndigentType::Other,
        }
    }
}

/// An indigent of a recipe.
/// This struct is used to store each indigents in a database. These can be attached to recipes.
#[derive(Debug, Clone)]
pub struct Indigent {
    pub name: String,
    indigent_id: u32,
    pub indigent_type: IndigentType,
}

impl Indigent {
    /// Create a new indigent.
    pub fn new(name: &str, indigent_id: u32, indigent_type: IndigentType) -> Indigent {
        Indigent {
            name: name.to_owned(),
            indigent_id,
            indigent_type,
        }
    }

    /// Indigent's IDs are hidden, because they are automatically created.
    pub fn indigent_id(&self) -> u32 {
        self.indigent_id
    }
}

impl PartialEq for Indigent {
    fn eq(&self, other: &Self) -> bool {
        self.indigent_id == other.indigent_id
    }
}