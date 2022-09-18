mod cache;
mod pseudo_database;
mod saliny;

fn main() {
    loop {
        let user_input = get_user_input(
            "Select what you want to do:
        1 CacheBooger
        2 PseudoDatabase
        3 Saliny",
        );

        let user_input: u8 = match user_input.trim().parse::<u8>() {
            Ok(T) => T,
            Err(E) => {
                println!("Error: {}", E);
                continue;
            }
        };

        match user_input {
            1 => cache::cache_booger(),
            2 => pseudo_database::start_database_interaction(),
            3 => saliny::begin_saliny(),
            _ => continue,
        }
    }
}

// UwU <3
fn get_user_input(display_text: &str) -> String {
    println!("{}\n", display_text);

    let mut buffer: String = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Invalid type");
    buffer.trim().to_string()
}
