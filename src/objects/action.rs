use std::collections::HashMap;

pub enum ActionType {
    Buy,
    Attack,
    Move,
    Research,
}

pub struct Action {
    pub id: String,
    pub player_id: String,
    pub exec_id: String,
    pub kind: ActionType,
    pub data: HashMap<String, String>,
}

impl Action {
    pub fn new(_id: &str, _player_id: &str, _exex_id: &str, _kind: ActionType, data: HashMap<String, String>) -> Action {
        let (id, player_id, exec_id, kind) = (_id.to_string(), _player_id.to_string(), _exex_id.to_string(), _kind);
        Action { id, player_id, exec_id, kind, data }
    }

    pub fn get_value(&self, key: &str) -> String {
        let hash_value = self.data.get(key);
        let value: String;

        match hash_value {
            Some(val) => value = val.to_string(),
            None      => panic!("Key doesn't exist for this action")
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_get_value() {
        let hash: HashMap<String, String> = [
            (String::from("hello"), String::from("32")),
            (String::from("world"), String::from("64"))
        ].iter().cloned().collect();

        let action = Action::new("1", "1", "1", ActionType::Buy, hash.clone());

        assert_eq!(action.get_value(&String::from("hello")), "32");
        assert_eq!(action.get_value(&String::from("world")), "64");
    }

    #[test]
    #[should_panic]
    fn test_get_value_panics() {
        let hash: HashMap<String, String> = [
            (String::from("hello"), "32".to_string()),
            (String::from("world"), "64".to_string())
        ].iter().cloned().collect();

        let action = Action::new("1", "1", "1", ActionType::Buy, hash.clone());

        action.get_value(&String::from("!"));
    }

}
