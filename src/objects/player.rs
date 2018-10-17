use crate::objects::action::*;

pub struct Player {
    pub id: String,
    pub name: String,
    pub money: u32,
    pub nodes: Vec<String>,
    pub execs: Vec<String>,
    pub actions: Vec<Action>,
}

impl Player {
    pub fn new(id: String, name: String, money: u32) -> Player {
        let nodes: Vec<String> = Vec::new();
        let execs: Vec<String> = Vec::new();
        let actions: Vec<Action> = Vec::new();

        Player { id, name, money, nodes, execs, actions }
    }

    pub fn add_node(&mut self, node_id: &String) {
        self.nodes.push(node_id.to_string());
    }
    pub fn remove_node(&mut self, node_id: &String) {
        self.nodes.remove_item(node_id);
    }

    pub fn add_exec(&mut self, exec_id: &String) {
        self.execs.push(exec_id.to_string());
    }
    pub fn remove_exec(&mut self, exec_id: &String) {
        self.execs.remove_item(exec_id);
    }

    pub fn add_money(&mut self, money: &u32) {
        self.money += money;
    }
    pub fn remove_money(&mut self, money: &u32) {
        self.money -= money;
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }
    pub fn remove_action(&mut self, a_id: &str) {
        self.actions.retain(|a| a.id != a_id);
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn add_node() {
        let node = String::from("new_node");
        let mut obj = Player::new(String::from("1"), String::from("Egon"), 0);
        obj.add_node(&node);
        assert_eq!(obj.nodes[0], node);
    }

    #[test]
    fn remove_node() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"),0);
        obj.add_node(&String::from("new_node"));
        obj.remove_node(&String::from("new_node"));
        assert_eq!(obj.nodes.len(), 0);
    }

    #[test]
    fn add_exec() {
        let exec = String::from("new_exec");
        let mut obj = Player::new(String::from("1"), String::from("Egon"),0);
        obj.add_exec(&exec);
        assert_eq!(obj.execs[0], exec);
    }

    #[test]
    fn remove_exec() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"),0);
        obj.add_exec(&String::from("new_exec"));
        obj.remove_exec(&String::from("new_exec"));
        assert_eq!(obj.execs.len(), 0);
    }

    #[test]
    fn add_money() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"),0);
        let old_money = obj.money.clone();
        let addition = 3000;
        obj.add_money(&addition);
        assert_eq!(obj.money, old_money+addition);
    }

    #[test]
    fn remove_money() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"), 3000);
        let old_money = obj.money.clone();
        let substraction = 3000;
        obj.remove_money(&substraction);
        assert_eq!(obj.money, old_money-substraction);
    }

    #[test]
    fn add_action() {
        use std::collections::HashMap;
        let mut obj = Player::new(String::from("1"), String::from("Egon"),0);
        let hm: HashMap<String, ActionData> = HashMap::new();
        let exec_id = "ex";
        let id = "1";
        let action = Action::new(id, &obj.id, exec_id, ActionType::Buy, hm);
        obj.add_action(action);
        assert_eq!(obj.actions[0].id, id.to_string());
    }

    #[test]
    fn remove_action() {
        use std::collections::HashMap;
        let mut obj = Player::new(String::from("1"), String::from("Egon"),0);
        let len = obj.actions.len();
        let hm: HashMap<String, ActionData> = HashMap::new();
        let exec_id = "ex";
        let id = "1";
        let action = Action::new(id, &obj.id, exec_id, ActionType::Buy, hm);
        obj.add_action(action);
        obj.remove_action(id);
        assert_eq!(obj.actions.len(), len);
    }

    #[test]
    fn reset_queue() {

    }
}
