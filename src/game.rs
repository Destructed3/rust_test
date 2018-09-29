use super::*;

pub fn run(game_data: GameData) {
    let gd = setup_players(game_data);
    let map = gd.map.iter();
    for row in map {
        let nodes = row.iter();
        for node in nodes {
            println!("node at {}/{}, ID: {}, Owner: {}", &node.x, &node.y, &node.id, &node.owner);
        }
    }

    let players = gd.players.iter();
    for player in players {
        println!("Player: {}; ID: {}, Nodes: {:?}, Execs: {:?}", &player.name, &player.id, &player.nodes, &player.execs);
    }

    let execs = gd.execs.iter();
    for exec in execs {
        println!("Exec: {}; ID: {}, Employer: {}", &exec.name, &exec.id, &exec.employer);
    }
}

fn setup_players(mut gd: GameData) -> GameData {
    for i in 0..gd.players.len() {
        let player_id = gd.players[i].id.clone();
        let mut exec_ids: Vec<String> = Vec::new();
        let mut node_id: String = String::from("");
        // FIND EXECS
        for _i in 0..2 {
            let execs = gd.execs.iter();
            for exec in execs {
                let id = exec_ids.iter().find(|id| id == &&exec.id);
                if exec.employer == "" && id == None {
                    exec_ids.push(exec.id.clone());
                    break;
                }
            }
        }
        for id in exec_ids {
            gd.add_exec_to_player(&id, &player_id);
        }
        // END FIND EXECS

        // FIND START NODE
        let rows = gd.map.iter();
        for row in rows {
            let nodes = row.iter();
            for node in nodes {
                if node.owner == "" {
                    node_id = node.id.clone();
                    break;
                }
            }
            if node_id.len() > 0 {
                break;
            }
        }
        gd.add_node_to_player(&node_id, &player_id);  
        // END FIND START NODE
    }

    gd
}

mod tests {
    use super::*;
    
    #[test]
    fn test_setup_players() {
        let dimensions = vec![16,16];
        let mut game_data = objects::game_data::GameData::new(&dimensions);
        
        game_data = setup_players(game_data);

        let _players: Vec<_> = game_data.players.iter().map( |player| {
            assert_eq!(player.execs.len(), 2);
            assert_eq!(player.nodes.len(), 1);
        }).collect();
    }
}