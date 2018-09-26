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
            let exec = Exec::new(id, generators::generate_name());
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

    pub fn get_exec(&mut self, id: &str) -> &mut Exec {
        let execs = self.execs.iter_mut().find(|e| e.id == id);

        match execs {
            Some(e) => return e,
            None    =>  {
                panic!("Didn't finde fitting exec!");
            }
        }
    }

    pub fn get_player(&mut self, id: &str) -> &mut Player {
        let players = self.players.iter_mut().find(|p| p.id == id);

        match players {
            Some(p) => return p,
            None    =>  {
                panic!("Didn't finde fitting exec!");
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_get_exec() {
        let dimensions = vec![1,1];
        let mut gd = GameData::new(&dimensions);

        let id = &gd.execs[0].id.to_string();
        let id2 = &gd.get_exec(&id).id;

        assert_eq!(id2, id);
    }

    #[test]
    #[should_panic]
    fn test_get_exec_panic() {
        let dimensions = vec![1,1];
        let mut gd = objects::game_data::GameData::new(&dimensions);

        gd.get_exec("!");
    }

        #[test]
    fn test_get_player() {
        let dimensions = vec![1,1];
        let mut gd = GameData::new(&dimensions);

        let id = &gd.players[0].id.to_string();
        let id2 = &gd.get_player(&id).id;

        assert_eq!(id2, id);
    }

    #[test]
    #[should_panic]
    fn test_get_player_panic() {
        let dimensions = vec![1,1];
        let mut gd = objects::game_data::GameData::new(&dimensions);

        gd.get_player("!");
    }
}