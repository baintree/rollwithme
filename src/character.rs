pub struct Character {
    pub race: Option<String>,
    pub class: Option<String>,
}

impl Character {
    pub fn new() -> Character{
        Character {
            race: None,
            class: None
        }
    }
}
