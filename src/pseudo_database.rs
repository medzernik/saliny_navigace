pub fn start_database_interaction() {
    create_pseudo_database();
}

///Gets user's text.
fn get_user_input(display_text: &str) -> String {
    println!("{}\n", display_text);

    let mut buffer: String = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Invalid type");
    buffer.trim().to_string()
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
        let user_input = get_user_input(
            "Select what you want to do:
        1 to add a new animal
        2 to find an animal by name
        3 to delete an animal by name",
        );

        let user_input: u8 = match user_input.trim().parse::<u8>() {
            Ok(T) => T,
            Err(E) => {
                println!("Error: {}", E);
                continue;
            }
        };

        match user_input {
            1 => create_animal(&mut database),
            2 => {
                let input_2 = get_user_input("Enter the search term\n");
                let result_of_search = match search_animal(&database, &input_2) {
                    Some(T) => T,
                    None => {
                        println!("No animal found");
                        continue;
                    }
                };

                println!("Result: {:?}", result_of_search.1);
            }
            3 => delete_animal(&mut database),
            _ => continue,
        }
    }
}

fn create_animal(database: &mut Vec<Animal>) {
    //get name - name is also the primary key, duplicity check below.
    let mut user_input_name;
    loop {
        user_input_name = get_user_input("Please put the name of the animal");
        if database.iter().any(|x| x.name == user_input_name) {
            println!("Name already in database, please enter a new name");
        } else {
            break;
        }
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

fn search_animal<'a>(
    database: &'a Vec<Animal>,
    animal_name: &'a String,
) -> Option<(usize, &'a Animal)> {
    database
        .iter()
        .enumerate()
        .find(|&(_, x)| x.name == *animal_name)
}

fn delete_animal(database: &mut Vec<Animal>) {
    let user_input_name = get_user_input("Please put the name of the animal you wish to delete");
    match search_animal(database, &user_input_name) {
        Some((index, _)) => {
            database.remove(index);
            println!("Animal deleted");
        }
        None => println!("{} not in database", &user_input_name),
    }
}
