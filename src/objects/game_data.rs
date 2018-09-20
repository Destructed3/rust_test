use super::*;
use crate::*;

pub struct GameData {
    pub map: Vec<Vec<Node>>,
    pub players: Vec<Player>,
    pub execs: Vec<Exec>,
    pub actions: Vec<Action>,
}
impl GameData {
    pub fn new(dimensions: &[u32]) -> GameData {
        // Create Players
        let mut players = Vec::new();
        for player_nr in 0..4 {
            let mut id = String::from("P");
            id.push_str(&player_nr.to_string());
            let player = Player::new(id, generators::generate_name());
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

        let actions = Vec::new();

        GameData { map, players, execs, actions }
    }
}