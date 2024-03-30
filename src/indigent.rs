use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum IndigentType {
    Vegetable,
    Fruit,
    Seed,
    Dairy,
    Meat,
    Fish,
    #[default]
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

    pub fn to_u32(&self) -> u32 {
        match self {
            IndigentType::Vegetable => 1,
            IndigentType::Fruit => 2,
            IndigentType::Seed => 3,
            IndigentType::Dairy => 4,
            IndigentType::Meat => 5,
            IndigentType::Fish => 6,
            IndigentType::Other => 7,
        }
    }
}

/// An indigent of a recipe.
/// This struct is used to store each indigents in a database. These can be attached to recipes.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

    /// Create a vector of indigents from a file.
    /// The file must be a .csv file with the following format:
    /// name;type
    ///
    /// # Examples
    ///
    /// ```
    /// let indigents = Indigent::from_file("indigents.csv");
    /// ```
    pub fn from_file(file_name: &str) -> Vec<Indigent> {
        let mut indigents = Vec::new();
        let file = std::fs::File::open(file_name).expect("Failed to open file");
        let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(file);

        for record in reader.deserialize() {
            let indigent: Indigent = record.expect("Failed to read record");
            indigents.push(indigent);
        }

        indigents
    }
}

impl PartialEq for Indigent {
    fn eq(&self, other: &Self) -> bool {
        self.indigent_id == other.indigent_id
    }
}
