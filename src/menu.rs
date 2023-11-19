use std::io;
use log::info;

pub fn draw_menu(db: &mut crate::database::RecipeDatabase) {
    println!("Welcome to the Recipe Generator!");
    println!("1. Indigent menu");
    println!("2. Recipe menu");
    println!("3. Generate a daily menu");
    println!("4. Generate a weekly menu");
    println!("5. Generate shopping list for a weekly menu");
    println!("6. Help");
    println!("7. Exit");

    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => {
                indigent_menu(db);
                break;
            },
            2 => {
                recipe_menu();
                break;
            },
            3 => {
                generate_daily_menu();
                break;
            },
            4 => {
                generate_weekly_menu();
                break;
            },
            5 => {
                generate_shopping_list();
                break;
            },
            6 => {

                break;
            },
            7 => {
                println!("Exit");
                info!("Goodbye!");
                break;
            },
            _ => {
                println!("Please enter a number between 1 and 7");
                continue;
            },
        }
    }

}

fn indigent_menu(db: &mut crate::database::RecipeDatabase) {
    println!("*********************************");
    println!("Indigent menu");

    println!("1. Add an indigent");
    println!("2. Remove an indigent");
    println!("3. List all indigents");
    println!("4. Back");

    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => {
                add_indigent(db);
                break;
            },
            2 => {
                remove_indigent(db);
                break;
            },
            3 => {
                list_all_indigents(db);
                break;
            },
            4 => {
                draw_menu(db);
                break;
            },
            _ => {
                println!("Please enter a number between 1 and 4");
                continue;
            },
        }
    }
}

fn add_indigent(db: &mut crate::database::RecipeDatabase) {
    println!("*********************************");
    println!("Add an indigent");
    println!();
    println!("Please enter the name of the indigent");
    let mut input_name = String::new();
    io::stdin().read_line(&mut input_name).expect("Failed to read line");

    let name = input_name.trim();

    println!("Please enter the type of the indigent");
    println!("1. Vegetable");
    println!("2. Fruit");
    println!("3. Seed");
    println!("4. Dairy");
    println!("5. Meat");
    println!("6. Fish");
    println!("7. Other");
    let mut input_type = String::new();
    io::stdin().read_line(&mut input_type).expect("Failed to read line");

    let input_as_num: u32 = match input_type.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number between 1 and 7");
            return;
        },
    };

    let type_of_indigent = crate::indigent::IndigentType::from_u32(input_as_num);

    let indigent = crate::indigent::Indigent::new(name, 0,type_of_indigent);

    db.add_indigent(indigent);

    info!("Indigent {} added to the database.", name);
    indigent_menu(db);
}

/// Removes an indigent from the database by its ID.
fn remove_indigent(db: &mut crate::database::RecipeDatabase) {
    println!("*********************************");
    println!("Remove an indigent");
    print!("Please enter the ID of the indigent: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            return;
        },
    };

    db.remove_indigent(input);

    info!("Indigent {} removed from the database.", input);
    indigent_menu(db);
}

fn list_all_indigents(db: &mut crate::database::RecipeDatabase) {
    println!("*********************************");
    println!("List all indigents");
    let indigents = db.get_all_indigents();
    for indigent in indigents {
        println!("{}: {}", indigent.indigent_id(), indigent.name);
    }

    indigent_menu(db);
}

fn recipe_menu() {
    println!("*********************************");
    println!("Recipe menu");
}

fn generate_daily_menu() {
    println!("Generate a daily menu");
}

fn generate_weekly_menu() {
    println!("Generate a weekly menu");
}

fn generate_shopping_list() {
    println!("Generate shopping list for a weekly menu");
}

fn help() {
    println!("Help");
}
