mod constants;
mod character;

use character::Character;
use inquire::Select;

fn main() {
    let character = Character {
        race: String::from(""),
        class: String::from("")
    };

    let ans = Select::new("What is your character's race? (Select \"Other\" if creating a custom race)", constants::RACES.to_vec()).prompt();

    match ans {
        Ok(choice) => println!("{}", assign_race(choice, character)),
        Err(_) => println!("There was an error reading your choice"),
    }
}

fn assign_race(choice: &str, mut character: Character) -> String {
    character.race = choice.to_string();
    character.race
}
