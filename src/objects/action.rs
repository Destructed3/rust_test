use std::collections::HashMap;

pub struct Action {
    pub id: String,
    pub player_id: String,
    pub exec_id: String,
    pub data: HashMap<String, ActionData>,
}
impl Action {
    pub fn new(_pid: &str, _eid: &str) -> Action {
        let id = [_pid, _eid].join("");
        let player_id = _pid.to_string();
        let exec_id = _eid.to_string();
        let data: HashMap<String, ActionData> = HashMap::new();
        Action { id, player_id, exec_id, data }
    }
}

pub enum ActionData {
    Type(ActionType),
    Text(String),
    Number(u32),
    Bool(bool),
    None,
}
impl ActionData {
    pub fn is_type(&self) -> bool {
        match self {
            ActionData::Type(_) => true,
            _ => false
        }
    }
    pub fn get_type(&self) -> &ActionType {
        match self {
            ActionData::Type(t) => return t,
            _ => panic!("No fitting value.")
        }
    }
    pub fn is_text(&self) -> bool {
        match self {
            ActionData::Text(_) => return true,
            _ => return false,
        }
    }
    pub fn get_text(&self) -> &str {
        match self {
            ActionData::Text(t) => t,
            _ => panic!("No fitting value"),
        }
    }
    pub fn is_number(&self) -> bool {
        match self {
            ActionData::Number(_) => true,
            _ => false
        }
    }
    pub fn get_number(self) -> u32 {
        match self {
            ActionData::Number(n) => n,
            _ => panic!("No fitting value."),
        }
    }
    pub fn is_bool(self) -> bool {
        match self {
            ActionData::Bool(_) => true, 
            _ => false,
        }
    }
    pub fn get_bool(self) -> bool {
        match self {
            ActionData::Bool(b) => b,
            _ => panic!("No fitting value."),
        }
    }
    pub fn is_empty(self) -> bool {
        match self {
            ActionData::None => true,
            _ => false,
        }
    }
}

pub enum ActionType {
    Buy,
}

#[cfg(test)]
mod tests {
    use super::*;
}
