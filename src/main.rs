use regex::{Captures, Regex};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let regex = Regex::new(r"\{{2}\s*(.*?)\s*\}{2}")?;

    let template = "Hello {{my}} world. {{ item in items }} {{ ddd }}";
    regex.captures_iter(&template).for_each(|caputers|{
        dbg!(&caputers[0]);     
        //&caputers[0] = "{{my}}"
        //&caputers[0] = "{{ Item in items }}"
        //&caputers[0] = "{{ ddd }}"
    });
 
    let template = regex.replace_all(template, |caps: &Captures| {
        format!("{}", &caps[1])
    });

    dbg!(template);
    //template = "Hello my world. Item in items ddd"

    Ok(())
}
