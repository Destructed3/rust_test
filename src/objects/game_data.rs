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
                panic!("Didn't find fitting exec!");
            }
        }
    }

    pub fn get_player(&mut self, id: &str) -> &mut Player {
        let players = self.players.iter_mut().find(|p| p.id == id);

        match players {
            Some(p) => return p,
            None    =>  {
                panic!("Didn't find fitting player!");
            }
        }
    }

    pub fn unemployed_execs(&mut self) -> Vec<&mut Exec> {
        let mut execs: Vec<&mut Exec> = Vec::new();
        
        let _execs = self.execs.iter_mut();
        
        for exec in _execs {
            if exec.employer == "" {
                execs.push(exec);
            }
        }

        if execs.len() < 1 {
            panic!("WHY GOD WHY?!");
        }

        execs
    }

    pub fn get_node(&mut self, id: &str) -> &mut Node {
        let rows = self.map.iter_mut();
        let mut node: Option<&mut Node> = None;
        for row in rows {
            node = row.iter_mut().find(|n| n.id == id);
            match &node {
                Some(_n) => break,
                None     => ()
            }
        }

        match node {
            Some(n) => return n,
            None    => panic!("No node found")
        }
    }

    pub fn add_exec_to_player(&mut self, exec_id: &str, player_id: &str) {
        self.get_player(player_id).execs.push(exec_id.to_string());
        self.get_exec(exec_id).employer = player_id.to_string();
    }
    pub fn remove_exec_from_player(&mut self, exec_id: &str, player_id: &str) {
        let p_execs = &mut self.get_player(player_id).execs;
        let index = p_execs.iter().position(|id| id == exec_id).unwrap();
        p_execs.remove(index);
        self.get_exec(exec_id).employer = String::from("");
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

    #[test]
    fn test_unemployed_execs() {
        let mut gd = GameData::new(&vec![1,1]);
        let execs_nr = gd.execs.iter().len();

        let unemp = gd.unemployed_execs();

        assert_eq!(execs_nr, unemp.len());
        
    }

    #[test]
    #[should_panic]
    fn test_unemployed_execs_panic() {
        let mut gd = GameData::new(&vec![1,1]);
        let execs = gd.execs.iter_mut();
        for exec in execs {
            exec.employer = String::from("Test");
        }

        gd.unemployed_execs();
    }

    #[test]
    fn test_get_node() {
        let mut gd = GameData::new(&vec![1,1]);
        let id = gd.map[0][0].id.to_owned();
        let node = gd.get_node(&id);
        assert_eq!(node.id, id);
    }

    #[test]
    #[should_panic]
    fn test_get_node_panic() {
        let mut gd = GameData::new(&vec![1,1]);
        gd.get_node("ERROR");
    }

    #[test]
    fn test_add_exec_to_player() {
        let mut gd = GameData::new(&vec![1,1]);
        let p_id = gd.players[0].id.clone();
        let e_id = gd.execs[0].id.clone();

        gd.add_exec_to_player(&e_id, &p_id);

        assert_eq!(gd.players[0].execs[0], e_id);
        assert_eq!(gd.execs[0].employer, p_id);
    }

    #[test]
    fn test_remove_exec_from_player() {
        let mut gd = GameData::new(&vec![1,1]);
        let p_id = gd.players[0].id.clone();
        let e_id = gd.execs[0].id.clone();

        let nr_e = gd.players[0].execs.len();

        gd.add_exec_to_player(&e_id, &p_id);
        gd.remove_exec_from_player(&e_id, &p_id);

        assert_eq!(gd.players[0].execs.len(), nr_e);
        assert_eq!(gd.execs[0].employer, "");
    }
}
