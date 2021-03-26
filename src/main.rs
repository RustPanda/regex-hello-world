use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let regex = Regex::new(r"\{{2}\s*(.*?)\s*\}{2}")?;

    regex.captures_iter("Hello {{my}} world. {{ item in items }} {{ ddd }}").for_each(|caputers|{
        dbg!(&caputers[1]);
        // [src/main.rs:10] &caputers[1] = "my"
        // [src/main.rs:10] &caputers[1] = "item in items"
        // [src/main.rs:10] &caputers[1] = "ddd"       
    });
 
    Ok(())
}
