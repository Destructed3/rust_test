use crate::*;

pub struct GameData {
    pub map: Vec<Vec<Node>>,
    pub players: Vec<Player>,
    pub execs: Vec<Exec>,
    pub actions: Vec<Action>,
    pub config: Config,
}

impl GameData {
    pub fn new(config: Config) -> GameData {
        // Create Players
        let mut players = Vec::new();
        for player_nr in 0..config.player_number {
            let mut id = String::from("P");
            id.push_str(&player_nr.to_string());
            let player = Player::new(id, generators::generate_name(), config.starting_money);
            players.push(player);
        }

        // Create Execs
        let mut execs = Vec::new();
        for exec_nr in 0..config.starting_execs() {
            let mut id = String::from("E");
            id.push_str(&exec_nr.to_string());
            let exec = Exec::new(id, generators::generate_name());
            execs.push(exec);
        }

        // Create map
        let mut map = Vec::with_capacity(config.map_size("y"));
        while map.len() < map.capacity() {
            let mut row = Vec::with_capacity(config.map_size("x"));
            while row.len() < row.capacity() {
                let coordinates = vec![row.len() as u32, map.len() as u32];
                let node = Node::new(&coordinates);
                row.push(node);
            }
            map.push(row);
        }

        let actions = Vec::new();

        GameData { map, players, execs, actions, config }
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
            panic!("Didn't find any unemployed execs");
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

    pub fn add_node_to_player(&mut self, node_id: &str, player_id: &str) {
        self.get_player(player_id).nodes.push(node_id.to_string());
        self.get_node(node_id).owner = player_id.to_string();
    }
    pub fn remove_node_from_player(&mut self, node_id: &str, player_id: &str) {
        let p_nodes = &mut self.get_player(player_id).nodes;
        let index = p_nodes.iter().position(|id| id == node_id).unwrap();
        p_nodes.remove(index);
        self.get_node(node_id).owner = String::from("");
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_get_exec() {
        let config = Config::new(vec![1,1], 1, 0, 1, 1, 1, 2);
        let mut gd = GameData::new(config);

        let id = &gd.execs[0].id.to_string();
        let id2 = &gd.get_exec(&id).id;

        assert_eq!(id2, id);
    }

    #[test]
    #[should_panic]
    fn test_get_exec_panic() {
        let config = Config::new(vec![1,1], 1, 0, 1, 1, 1, 1);
        let mut gd = objects::game_data::GameData::new(config);

        gd.get_exec("!");
    }

    #[test]
    fn test_get_player() {
        let config = Config::new(vec![1,1], 4, 0, 1, 1, 1, 1);
        let mut gd = GameData::new(config);

        let id = &gd.players[0].id.to_string();
        let id2 = &gd.get_player(&id).id;

        assert_eq!(id2, id);
    }

    #[test]
    #[should_panic]
    fn test_get_player_panic() {
        let config = Config::new(vec![1,1], 1, 0, 1, 1, 1, 1);
        let mut gd = objects::game_data::GameData::new(config);

        gd.get_player("!");
    }

    #[test]
    fn test_unemployed_execs() {
        let config = Config::new(vec![1,1], 4, 0, 1, 1, 1, 1);
        let mut gd = GameData::new(config);
        let execs_nr = gd.execs.iter().len();

        let unemp = gd.unemployed_execs();

        assert_eq!(execs_nr, unemp.len());
        
    }

    #[test]
    #[should_panic]
    fn test_unemployed_execs_panic() {
        let config = Config::new(vec![1,1], 1, 0, 1, 1, 1, 1);
        let mut gd = GameData::new(config);
        let execs = gd.execs.iter_mut();
        for exec in execs {
            exec.employer = String::from("Test");
        }

        gd.unemployed_execs();
    }

    #[test]
    fn test_get_node() {
        let config = Config::new(vec![1,1], 1, 0, 1, 1, 1, 1);
        let mut gd = GameData::new(config);
        let id = gd.map[0][0].id.to_owned();
        let node = gd.get_node(&id);
        assert_eq!(node.id, id);
    }

    #[test]
    #[should_panic]
    fn test_get_node_panic() {
        let config = Config::new(vec![1,1], 1, 0, 1, 1, 1, 1);
        let mut gd = GameData::new(config);
        gd.get_node("ERROR");
    }

    #[test]
    fn test_add_exec_to_player() {
        let config = Config::new(vec![1,1], 1, 0, 1, 1, 1, 1);
        let mut gd = GameData::new(config);
        let p_id = gd.players[0].id.clone();
        let e_id = gd.execs[0].id.clone();

        gd.add_exec_to_player(&e_id, &p_id);

        assert_eq!(gd.players[0].execs[0], e_id);
        assert_eq!(gd.execs[0].employer, p_id);
    }

    #[test]
    fn test_remove_exec_from_player() {
        let config = Config::new(vec![1,1], 1, 0, 1, 1, 1, 1);
        let mut gd = GameData::new(config);
        let p_id = gd.players[0].id.clone();
        let e_id = gd.execs[0].id.clone();

        let nr_e = gd.players[0].execs.len();

        gd.add_exec_to_player(&e_id, &p_id);
        gd.remove_exec_from_player(&e_id, &p_id);

        assert_eq!(gd.players[0].execs.len(), nr_e);
        assert_eq!(gd.execs[0].employer, "");
    }

    #[test]
    fn test_add_node_to_player() {
        let config = Config::new(vec![1,1], 1, 0, 1, 1, 1, 1);
        let mut gd = GameData::new(config);
        let p_id = gd.players[0].id.clone();
        let n_id = gd.map[0][0].id.clone();

        let nr_n = gd.players[0].nodes.len();

        gd.add_node_to_player(&n_id, &p_id);

        assert!(gd.players[0].nodes.len() > nr_n);
        assert_eq!(gd.players[0].nodes[0], n_id);
        assert_eq!(gd.map[0][0].owner, p_id);
    }

    #[test]
    fn test_remove_node_from_player() {
        let config = Config::new(vec![1,1], 1, 0, 1, 1, 1, 1);
        let mut gd = GameData::new(config);
        let p_id = gd.players[0].id.clone();
        let n_id = gd.map[0][0].id.clone();

        let nr_n = gd.players[0].nodes.len();

        gd.add_node_to_player(&n_id, &p_id);
        gd.remove_node_from_player(&n_id, &p_id);

        assert_eq!(gd.players[0].nodes.len(), nr_n);
        assert_eq!(gd.map[0][0].owner, "".to_string());
    }
}
