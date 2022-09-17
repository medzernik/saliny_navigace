use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn start_database_interaction() {
    create_pseudo_database();
}

struct Animal {
    name: String,
    species: String,
    age: u16,
    weight: f64,
}

fn create_pseudo_database() {
    let mut database: Vec<Animal> = Vec::new();

    loop {
        println!(
            "Select what you want to do:\n
        1 to enter a new animal\n
        2 to find an animal by name\n
        3 to delete an animal by name"
        );

        let input: i32 = get_user_input(
            "Select what you want to do:\n
        1 to enter a new animal\n
        2 to find an animal by name\n
        3 to delete an animal by name",
        )
        .input;

        println!("Input was: {}", input);

        /*
        match input {
            1 => {}
            2 => {}
            3 =>{}

        }

         */
    }
}

struct UserInput<T> {
    input: T,
}

///should be generic and write out stuff. Which it does right now.
fn get_user_input<T>(display_text: &str) -> UserInput<T> {
    println!("{}", display_text);
    let mut buffer = String::new();

    let mut input: String = std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line").to_string();

    let input = input.parse().expect("Invalid input type");


    return input;
}

fn create_animal(database: &mut Vec<Animal>) {
    let user_input = get_user_input("Please put the name of the animal").input;
    println!("Input was: {}", user_input);
    let animal1 = Animal {
        name: "Otter".to_string(),
        species: "Otterus".to_string(),
        age: 2,
        weight: 15.6,
    };
    database.push(animal1);
}

fn search_animal(database: &mut Vec<Animal>) {}

fn delete_animal(database: &mut Vec<Animal>) {}
