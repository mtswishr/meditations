use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::prelude::*;
// A program that will randomly output some of Marcus' meditations to standard output. 
// Mostly doing this to satisfy some aesthetic reasoning on terminal start.
fn main() {
    let path = Path::new("data/meditations.txt");
    let display = path.display();

    let mut file  = match File::open(&path) {
        Err(why) => panic!("couldn't read the path: {} for this reason: {}", display, why),
        Ok(file) => file,
    };

    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => println!("********************************************************"),
    }

    let mut rng = rand::thread_rng();
    let _base: u32 = rng.gen::<u32>();
    let split_text: Vec<&str> = text.split("\n\n").collect();
    let passage = split_text.choose(&mut rng);

    match passage {
        Some(text) => println!("{}", text),
        None => println!("Get stronger today")
    }
    println!("********************************************************");
}
