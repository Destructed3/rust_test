#![feature(vec_remove_item)]

pub struct Node {
    pub id: String,
    pub x: u32,
    pub y: u32,
    pub owner: String,
}
impl Node {
    pub fn new(coordinates: &[u32]) -> Node {
        let x = coordinates[0].clone();
        let y = coordinates[1].clone();
        let mut id = String::from("N");
        id.push_str(&x.to_string());
        id.push_str(&y.to_string());
        let owner = String::from("");

        Node { id, x, y, owner }
    }

    pub fn change_owner(&mut self, new_owner: String) {
        self.owner = new_owner;
    }
}

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

    pub fn add_node(&mut self, node_id: String) {
        self.nodes.push(node_id);
    }

    pub fn remove_node(&mut self, node_id: &String) {
        self.nodes.remove_item(node_id);
    }

    pub fn add_exec(&mut self, exec_id: String) {
        self.execs.push(exec_id);
    }

    pub fn remove_exec(&mut self, exec_id: &String) {
        self.execs.remove_item(exec_id);
    }
}

pub struct Exec {
    pub id: String,
    pub name: String,
    pub employer: String,
}
impl Exec {
    pub fn new(id: String, name: String) -> Exec {
        let employer = String::from("");
        
        Exec { id, name, employer }
    }
}

pub struct GameLogic {
    pub map: Vec<Vec<Node>>,
    pub players: Vec<Player>,
    pub execs: Vec<Exec>,
}
impl GameLogic {
    pub fn new(dimensions: &[u32]) -> GameLogic {
        // Create Players
        let mut players = Vec::new();
        for player_nr in 0..4 {
            let mut id = String::from("P");
            id.push_str(&player_nr.to_string());
            let player = Player::new(id, String::from("Egon"));
            players.push(player);
        }

        // Create Execs
        let mut execs = Vec::new();
        for exec_nr in 0..12 {
            let mut id = String::from("E");
            id.push_str(&exec_nr.to_string());
            let exec = Exec::new(id, String::from("Ray"));
            execs.push(exec);
        }

        // Create map
        let mut map = Vec::new();
        for x in 0..dimensions[0] {
            let mut row = Vec::new();
            for y in 0..dimensions[1] {
                let coordinates = [x.clone(), y.clone()].to_vec();
                let node = Node::new(&coordinates);
                row.push(node);
            }
            map.push(row);
        }

        GameLogic { map, players, execs }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"));
        obj.add_node(String::from("new_node"));
        assert!(obj.nodes[0] == String::from("new_node"));
    }

    #[test]
    fn test2() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"));
        obj.add_node(String::from("new_node"));
        obj.remove_node(&String::from("new_node"));
        assert_eq!(obj.nodes.len(), 0);
    }

    #[test]
    fn test3() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"));
        obj.add_exec(String::from("new_exec"));
        assert!(obj.execs[0] == String::from("new_exec"));
    }

    #[test]
    fn test4() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"));
        obj.add_exec(String::from("new_exec"));
        obj.remove_exec(&String::from("new_exec"));
        assert_eq!(obj.execs.len(), 0);
    }
}