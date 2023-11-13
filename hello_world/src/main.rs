//for this module you can build a project with: cargo build
//run your cargo with : cargo run
//test your project with : cargo test 
//build documentation for your project with: cargo doc
//and can be published with crates.io using: cargo publish


//the greetings module is associated with this crate
mod greetings; //the mod was what was used to bring the file into the crate with main
//use greetings::default_greeting; //associating the module with the crate and making the greetings file the default greeting
use greetings::{default_greeting, spanish, french};

fn main() {  //The main function
    println!("Hello, world!"); //printing hello world
    println!("{}", default_greeting());//will still print hello world as hello world is now the default greeting; this was added when the contents of the greeting folder were created
    println!("{}", spanish::default_greeting());//function declarations for spanisha nd french files respectively
    println!("{}", french::default_greeting());
}
//refer back to Lesson 3 Slide 43 i.e 16 for the website he linked, also note the instructions for library and binary creations

