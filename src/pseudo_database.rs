

pub fn start_database_interaction() {
    create_pseudo_database();
}

#[derive(Debug)]
struct Animal {
    name: String,
    species: String,
    age: u16,
    weight: f64,
}

fn create_pseudo_database() {
    let mut database: Vec<Animal> = Vec::new();

    loop {

        let input = get_user_input(
            "Select what you want to do:\n
        1 to enter a new animal\n
        2 to find an animal by name\n
        3 to delete an animal by name",
        );

        let input: u8 = input.trim().parse().expect("Not a number");

        println!("\nInput was: {}\n", input);


        match input {
            1 => {create_animal(&mut database)}
            2 => {search_animal(&database)}
            3 => {delete_animal(&mut database)}
            _ => continue

        }


    }
}


///should be generic and write out stuff. Which it does right now.
fn get_user_input(display_text: &str) -> String {
    println!("{}", display_text);

    let mut buffer: String = String::new();
    let  input: String = std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line").to_string();

    let input: String = input.trim().parse().unwrap();

    return input;
}

fn create_animal(database: &mut Vec<Animal>) {
    //get name
    let user_input_name = get_user_input("Please put the name of the animal");
    //get species
    let user_input_species = get_user_input("Please put the name of the animal");
    //get age and convert
    let user_input_age = get_user_input("Please put the name of the animal");
    let user_input_age: u16 = user_input_age.trim().parse().expect("Invalid symbol (outside u16");

    //get weight and convert
    let user_input_weight = get_user_input("Please put the name of the animal");
    let user_input_weight: f64 = user_input_weight.trim().parse().expect("Invalid symbol (outside f64");


    let animal1 = Animal {
        name: user_input_name,
        species: user_input_species,
        age: user_input_age,
        weight: user_input_weight,
    };


    println!("{:?}", animal1);
    database.push(animal1);
}

fn search_animal(database: &Vec<Animal>) {}

fn delete_animal(database: &mut Vec<Animal>) {}
