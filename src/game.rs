use super::*;

pub fn run(game_data: GameData) {
    let map = game_data.map.iter();
    for row in map {
        let nodes = row.iter();
        for node in nodes {
            println!("node at {}/{}, ID: {}", &node.x, &node.y, &node.id);
        }
    }

    let players = game_data.players.iter();
    for player in players {
        println!("Player: {}; ID: {}", &player.name, &player.id);
    }

    let execs = game_data.execs.iter();
    for exec in execs {
        println!("Exec: {}; ID: {}", &exec.name, &exec.id);
    }
}

fn setup_players(gd: &GameData) {
    /**
     * 1 find empty nodes
     * 2 add one of them to player
     * 3 find unemployed execs
     * 4 add one of them to player
     * 5-6 repeat 3-4
     * 
     * Do for every player
     */
    let mut players = gd.players.iter();
    for mut player in players {
        //let node = get_start_node(&gd);
        for _i in 0..2 {

        }
    }
}

// fn get_start_node(gd: &GameData) -> Node {
//     let node = gd.map[0][0];

//     node
// }

fn find_unemployed_execs_id(gd: &GameData) -> Vec<String> {
    let mut unemployed: Vec<String> = Vec::new();

    let execs = gd.execs.iter();
    for exec in execs {
        if exec.employer == "" {
            unemployed.push(exec.id.to_string());
        }
    }

    unemployed
}

//fn get_unemployed_exec(gd: &GameData) -> &Exec {
    //let unemployed = find_unemployed_execs_id(gd);

    //exec = gd.getunemployed[0]
//}

fn add_exec_to_player(exec: &mut Exec, player: &mut Player) {
    exec.change_employer(&player.id);
    player.add_exec(&exec.id);
}

fn remove_exec_from_player(exec: &mut Exec, player: &mut Player) {
    exec.change_employer(&String::from(""));
    player.remove_exec(&exec.id);
}

fn add_node_to_player(node: &mut Node, player: &mut Player) {
    node.change_owner(&player.id);
    player.add_node(&node.id);
}

fn remove_node_from_player(node: &mut Node, player: &mut Player) {
    node.change_owner(&String::from(""));
    player.remove_node(&node.id);
}

mod tests {
    use super::*;
    
    #[test]
    fn test_setup_players() {
        let dimensions = vec![16,16];
        let game_data = objects::game_data::GameData::new(&dimensions);
        
        setup_players(&game_data);

        let _players: Vec<_> = game_data.players.iter().map( |player| {
            assert_eq!(player.execs.len(), 2);
            assert_eq!(player.nodes.len(), 1);
        }).collect();
    }

    #[test]
    fn test_find_unemployed_execs_id() {
        let dimensions = vec![16,16];
        let mut game_data = objects::game_data::GameData::new(&dimensions);

        let mut exec = &mut game_data.execs[0];
        let mut player = &mut game_data.players[0];

        add_exec_to_player(&mut exec, &mut player);

        let unemployed = game::find_unemployed_execs_id(&game_data);            
        
        assert!(unemployed.len() > 0);
        assert_eq!(unemployed.len(), game_data.execs.len()-1);

        let mut ids = Vec::new();

        for nr in 0..game_data.execs.len() {
            let mut s = String::from("E");
            s.push_str(&nr.to_string());
            ids.push(s);
        }

        let exec_ids = unemployed.iter();
        
        for id in exec_ids {
            assert!(ids.contains(&id));
        }
    }

    #[test]
    fn test_add_exec_to_player() {
        let mut exec = Exec::new(String::from("E1"), String::from("Egon"));
        let mut player = Player::new(String::from("P1"), String::from("Egon"));

        add_exec_to_player(&mut exec, &mut player);

        assert!(player.execs.len() > 0);
        assert_eq!(&player.execs[0], &exec.id);
        assert_eq!(&player.id, &exec.employer);
    }
    #[test]
    fn test_remove_exec_from_player() {
        let mut exec = Exec::new(String::from("E1"), String::from("Egon"));
        let mut player = Player::new(String::from("P1"), String::from("Egon"));

        add_exec_to_player(&mut exec, &mut player);
        remove_exec_from_player(&mut exec, &mut player);

        assert!(player.execs.len() < 1);
        assert_eq!(&exec.employer, "");
    }

    #[test]
    fn test_add_node_to_player() {
        let mut node = Node::new(&vec![1,1]);
        let mut player = Player::new(String::from("P1"), String::from("Egon"));
        
        add_node_to_player(&mut node, &mut player);

        assert!(player.nodes.len() > 0);
        assert_eq!(player.nodes[0], node.id);
        assert_eq!(player.id, node.owner);
    }
    #[test]
    fn test_remove_node_from_player() {
        let mut node = Node::new(&vec![1,1]);
        let mut player = Player::new(String::from("P1"), String::from("Egon"));

        add_node_to_player(&mut node, &mut player);
        remove_node_from_player(&mut node, &mut player);

        assert!(player.nodes.len() < 1);
        assert_eq!(&node.owner, "");
    }
}