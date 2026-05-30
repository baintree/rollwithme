mod constants;
mod character;

use character::Character;
use inquire::Select;

fn main() {
    let mut character = Character {
        race: String::from(""),
        class: String::from("")
    };

    let ans = Select::new("What is your character's race? (Select \"Other\" if creating a custom race)", constants::RACES.to_vec()).prompt();

    match ans {
        Ok(choice) => {
            character.race = choice.to_string();
            prompt_classes(&mut character);
            println!("{}", character.race);
        },
        Err(_) => println!("There was an error reading your choice"),
    }
}

fn prompt_classes(character: &mut Character) {
    //TODO: Add call to fetch classes for this race and populate class propert of Character
}