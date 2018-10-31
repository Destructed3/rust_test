use super::action::Action;

pub trait ActionQueue {
    fn get_action(&self, aid: &str) -> &Action;
    fn remove_action(&mut self, aid: &str) -> bool;
}
pub struct Exec {
    pub id: String,
    pub name: String,
    pub employer: String,
    pub action_queue: Vec<Action>,
}
impl Exec {
    pub fn new(_id: &str, _name: &str) -> Exec {
        let id = _id.to_string();
        let name = _name.to_string();
        let employer = String::from("");
        let action_queue: Vec<Action> = Vec::new();
        Exec { id, name, employer, action_queue }
    }
}
impl ActionQueue for Exec {
    fn get_action(&self, aid: &str) -> &Action {
        self.action_queue.iter().find( |a| a.id == aid ).unwrap()
    }

    fn remove_action(&mut self, aid: &str) -> bool {
        let i_opt = self.action_queue.iter().position( |a| a.id == aid );
        match i_opt {
            Some(i) => {
                self.action_queue.remove(i);
                true
            },
            None => false
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_action() {
        let mut exec = Exec::new("E", "E");
        let action = Action::new("P", "E");
        let id = action.id.to_string();
        exec.action_queue.push(action);
        let result = exec.get_action(&id);
        assert!(result.id == id && result.exec_id == "E");
    }

    #[test]
    fn test_remove_action() {
        let mut exec = Exec::new("E", "E");
        let action = Action::new("P", "E");
        let id = action.id.to_string();
        exec.action_queue.push(action);
        assert_eq!(exec.action_queue.len(), 1);
        assert!( exec.remove_action(&id) );
        assert_eq!(exec.action_queue.len(), 0);
        assert!( !exec.remove_action(&id) );
    }
}