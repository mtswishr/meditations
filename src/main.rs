use rand::prelude::*;
// A program that will randomly output some of Marcus' meditations to standard output. 
// Mostly doing this to satisfy some aesthetic reasoning on terminal start.
fn main() {
    let meditations = include_str!("../data/meditations.txt");

    let mut rng = rand::thread_rng();
    let _base: u32 = rng.gen::<u32>();
    let split_text: Vec<&str> = meditations.split("\n\n").collect();
    let passage = split_text.choose(&mut rng);

    match passage {
        Some(text) => println!("{}", text),
        None => println!("Get stronger today")
    }
    println!("********************************************************");
}
