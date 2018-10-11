use std::collections::HashMap;

/// The Action-Type. Is supposed to hold all important information, about an action
pub struct Action {
    pub id: String,
    pub player_id: String,
    pub exec_id: String,
    pub kind: ActionType,
    pub data: HashMap<String, ActionData>,
}
impl Action {
    pub fn new(_id: &str, _player_id: &str, _exex_id: &str, _kind: ActionType, data: HashMap<String, ActionData>) -> Action {
        let (id, player_id, exec_id, kind) = (_id.to_string(), _player_id.to_string(), _exex_id.to_string(), _kind);
        Action { id, player_id, exec_id, kind, data }
    }

    pub fn get_value(&self, key: &str) -> &ActionData {
        let hash_value = self.data.get(key);
        match hash_value {
            Some(val) => return val,
            None      => panic!("Key doesn't exist for this action")
        }
    }
}

pub enum ActionType {
    Buy,
    Attack,
    Move,
    Research,
}
/// Holds dynamic data necessary for the action
/// holding Datatypes: 
/// * Text -> String
/// * Number -> u32
/// * Boolean -> bool
pub enum ActionData {
    Text(String),
    Number(u32),
    Boolean(bool),
    None
}
impl ActionData {
    pub fn is_text(&self) -> bool {
        match &self {
            ActionData::Text(_) => return true,
            ActionData::Number(_) => return false,
            ActionData::Boolean(_) => return false,
            ActionData::None => return false
        }
    }
    pub fn get_text(&self) -> &str {
        match &self {
            ActionData::Text(t) => return &t,
            ActionData::Number(_) => panic!("No fitting value."),
            ActionData::Boolean(_) => panic!("No fitting value."),
            ActionData::None => panic!("No fitting value.")
        }
    }
    pub fn is_number(&self) -> bool {
        match &self {
            ActionData::Text(_) => return false,
            ActionData::Number(_) => return true,
            ActionData::Boolean(_) => return false,
            ActionData::None => return false
        }
    }
    pub fn get_number(&self) -> &u32 {
        match &self {
            ActionData::Text(_) => panic!("No fitting value."),
            ActionData::Number(n) => return n,
            ActionData::Boolean(_) => panic!("No fitting value."),
            ActionData::None => panic!("No fitting value.")
        }
    }
    pub fn is_bool(&self) -> bool {
        match &self {
            ActionData::Text(_) => return false,
            ActionData::Number(_) => return false,
            ActionData::Boolean(_) => return true,
            ActionData::None => return false
        }
    }
    pub fn get_bool(&self) -> &bool {
        match &self {
            ActionData::Text(_) => panic!("No fitting value."),
            ActionData::Number(_) => panic!("No fitting value."),
            ActionData::Boolean(b) => return b,
            ActionData::None => panic!("No fitting value.")
        }
    }
    pub fn is_empty(self) -> bool {
        match self {
            ActionData::Text(_) => return false,
            ActionData::Number(_) => return false,
            ActionData::Boolean(_) => return false,
            ActionData::None => return true
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_ActionData_get_text() {
        let v = ActionData::Text(String::from("Test"));
        assert_eq!(v.get_text(), String::from("Test"));
    }
    #[test]
    #[should_panic]
    fn test_ActionData_get_text_panic() {
        let v = ActionData::Number(3);
        v.get_text();
    }
    #[test]
    fn test_ActionData_get_number() {
        let u: u32 = 3;
        let v = ActionData::Number(u);
        assert_eq!(v.get_number(), &u);
    }
    #[test]
    #[should_panic]
    fn test_ActionData_get_number_panic() {
        let v = ActionData::Text(String::from("Test"));
        v.get_number();
    }

    #[test]
    fn test_ActionData_get_bool() {
        let mut v = ActionData::Boolean(true);
        assert!(v.get_bool());
        v = ActionData::Boolean(false);
        assert!(!v.get_bool());
    }

    #[test]
    #[should_panic]
    fn test_ActionData_get_bool_panic() {
        let v = ActionData::Number(3);
        v.get_bool();
    }

    #[test]
    fn test_get_value() {
        let nr: u32 = 3;
        let txt: String = String::from("Test");
        let b: bool = true;
        let (key1, key2, key3) = ("hello", "world", "__");
        let mut hash: HashMap<String, ActionData> = HashMap::new();
        hash.insert(String::from(key1), ActionData::Boolean(b));
        hash.insert(String::from(key2), ActionData::Number(nr));
        hash.insert(String::from(key3), ActionData::Text(txt.to_string()));

        let action = Action::new("1", "1", "1", ActionType::Buy, hash);
        assert_eq!(action.get_value(key1).get_bool(), &b);
        assert_eq!(action.get_value(key2).get_number(), &nr);
        assert_eq!(action.get_value(key3).get_text(), txt);
    }

    #[test]
    #[should_panic]
    fn test_get_value_panics() {
        let hash: HashMap<String, ActionData> = HashMap::new();
        let action = Action::new("1", "1", "1", ActionType::Buy, hash);

        action.get_value(&String::from("!"));
    }

}
