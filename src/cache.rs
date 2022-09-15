use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;

pub fn cache_booger() {
    /*let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Please put in a number");*/

    //get_data(line_number);



    //println!("Result of cache: {}", get_data());

    let mut map: HashMap<i32, String> = HashMap::new();
    read_file_populate_cache(&mut map);

    let mut output = read_cache(&mut map);
    if output == "Not found" {
        println!("Populating cache, please wait...");
        read_file_populate_cache(&mut map);
        std::thread::sleep(std::time::Duration::from_secs(1));
        output = read_cache(&mut map);
        if output == "Not found" {
            println!("Still not found, sorry");
        } else {
            println!("Result of cache: {}", output);
        }
    }
    else {
        println!("Result of cache: {}", output);
    }
}

fn read_cache(map: &mut HashMap<i32, String>) -> &str{
    let mut input = String::new();

    println!("Input what you want");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Please put in a number");

    match map.get(&input) {
        Some(value) => value,
        None => "Not found",
    }


}

fn read_file_populate_cache(return_hash: &mut HashMap<i32, String>)  {
       let file = File::open(r"week-05\lecture-review.md").expect("error opening the file");
    let reader = BufReader::new(file);

    //old function
    for (i, line) in reader.lines().enumerate() {
        return_hash.insert(
            i as i32,
            line.expect(&format!(
                "Invalid value tried to be inserted to place {}",
                i
            )),
        );
    }
}
