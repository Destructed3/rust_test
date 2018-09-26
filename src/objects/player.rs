pub struct Player {
    pub id: String,
    pub name: String,
    pub money: u32,
    pub nodes: Vec<String>,
    pub execs: Vec<String>,
}

impl Player {
    pub fn new(id: String, name: String) -> Player {
        let money = 1000000;
        let nodes: Vec<String> = Vec::new();
        let execs: Vec<String> = Vec::new();

        Player { id, name, money, nodes, execs }
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
}

mod tests {
    use super::*;

    #[test]
    fn add_node() {
        let node = String::from("new_node");
        let mut obj = Player::new(String::from("1"), String::from("Egon"));
        obj.add_node(&node);
        assert_eq!(obj.nodes[0], node);
    }

    #[test]
    fn remove_node() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"));
        obj.add_node(&String::from("new_node"));
        obj.remove_node(&String::from("new_node"));
        assert_eq!(obj.nodes.len(), 0);
    }

    #[test]
    fn add_exec() {
        let exec = String::from("new_exec");
        let mut obj = Player::new(String::from("1"), String::from("Egon"));
        obj.add_exec(&exec);
        assert_eq!(obj.execs[0], exec);
    }

    #[test]
    fn remove_exec() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"));
        obj.add_exec(&String::from("new_exec"));
        obj.remove_exec(&String::from("new_exec"));
        assert_eq!(obj.execs.len(), 0);
    }

    #[test]
    fn add_money() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"));
        let old_money = obj.money.clone();
        let addition = 3000;
        obj.add_money(&addition);
        assert_eq!(obj.money, old_money+addition);
    }

    #[test]
    fn remove_money() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"));
        let old_money = obj.money.clone();
        let substraction = 3000;
        obj.remove_money(&substraction);
        assert_eq!(obj.money, old_money-substraction);
    }
}
