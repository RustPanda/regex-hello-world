use regex::{Captures, Regex};
use std::{collections::BTreeMap, error::Error, iter::FromIterator};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let regex = Regex::new(r"\{{2}\s*(\w+)\s*\}{2}")?;

    let map = BTreeMap::from_iter(vec![("my", "guys"), ("ddd", "End")]);

    let template = "Hello {{my}} world. {{ item in items }} {{ ddd }}";
    regex.captures_iter(&template).for_each(|caputers| {
        dbg!(&caputers[0]);     
        //&caputers[0] = "{{my}}"
        //&caputers[0] = "{{ ddd }}"
    });
 
    let template = regex.replace_all(template, |caps: &Captures| {
        format!("{}", map.get(&caps[1]).unwrap())
    });

    dbg!(template);
    //template = "Hello guys world. {{ item in items }} End"

    Ok(())
}
