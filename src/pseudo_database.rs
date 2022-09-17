pub fn start_database_interaction() {
    create_pseudo_database();
}
///Gets users' text.
fn get_user_input(display_text: &str) -> String {
    println!("{}", display_text);

    let mut buffer: String = String::new();
    let input: String = std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line")
        .to_string();

    let input: String = input.trim().parse().unwrap();

    return input;
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
        1 to add a new animal\n
        2 to find an animal by name\n
        3 to delete an animal by name",
        );

        let input: u8 = input.trim().parse().expect("Not a number");

        println!("\nInput was: {}\n", input);

        match input {
            1 => create_animal(&mut database),
            2 => {
                let input = get_user_input("Enter the search term\n");

                println!("Result: {:?}", search_animal(&database, input));
            }
            3 => delete_animal(&mut database),
            _ => continue,
        }
    }
}



fn create_animal(database: &mut Vec<Animal>) {
    //get name
    let mut user_input_name;
    loop{
        user_input_name = get_user_input("Please put the name of the animal");
        if database.iter().any(|x| x.name == user_input_name) {
            println!("Name already in database, please enter a new name");
        }
        else { break; }
    }
    //get species
    let user_input_species = get_user_input("Please put the species of the animal");
    //get age and convert
    let user_input_age = get_user_input("Please put the age of the animal");
    let user_input_age: u16 = user_input_age
        .trim()
        .parse()
        .expect("Invalid symbol (outside u16");

    //get weight and convert
    let user_input_weight = get_user_input("Please put the weight of the animal");
    let user_input_weight: f64 = user_input_weight
        .trim()
        .parse()
        .expect("Invalid symbol (outside f64");

    let animal1 = Animal {
        name: user_input_name,
        species: user_input_species,
        age: user_input_age,
        weight: user_input_weight,
    };

    println!("{:?}", animal1);
    database.push(animal1);
}

fn search_animal(database: &Vec<Animal>, animal_name: String) -> Option<(usize, &Animal)> {
    database.iter().enumerate().find(|&(_, x)| x.name == animal_name)
}

fn delete_animal(database: &mut Vec<Animal>) {
    let user_input_name = get_user_input("Please put the name of the animal you wish to delete");
    if database.iter().any(|x| x.name != user_input_name) {
        database.retain(|x| x.name != user_input_name);
        println!("Animal deleted");
    }
    else {
        println!("Name not in database");
    }

    //database = database.iter().filter(|x| {});
}
