mod character;
mod constants;

use character::Character;
use inquire::Select;

enum AppState {
    PromptRace,
    PromptClass,
    PromptSubclass,
    Finished,
}

fn main() {
    let mut current_state = AppState::PromptRace;
    let mut character = Character::new();
    while !matches!(current_state, AppState::Finished) {
        match current_state {
            AppState::PromptRace => {
                let ans = Select::new(
                    "What is your character's race? (Select \"Other\" if creating a custom race)",
                    constants::RACES.to_vec(),
                )
                .prompt();

                match ans {
                    Ok(choice) => {
                        character.race = Some(choice.to_string());
                        current_state = AppState::PromptClass;
                    }
                    Err(_) => println!("There was an error reading your choice"),
                }
            }
            AppState::PromptClass => {
                let ans = Select::new(
                    "What is your character's class?",
                    constants::CLASSES.to_vec(),
                )
                .prompt();

                match ans {
                    Ok(choice) => {
                        character.class = Some(choice.to_string());
                        current_state = AppState::PromptSubclass;
                    },
                    Err(_) => println!("There was an error reading your choice"),
                }
            }
            AppState::PromptSubclass => todo!(),
            AppState::Finished => todo!(),
        }
    }
}