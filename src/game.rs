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
        // END FIND EXECS

        // FIND START NODE
        {
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
        }
        // END FIND START NODE

        // Add Stuff together
        {
            let player_option = gd.players.iter_mut().find(|p| p.id == player_id);
            match player_option {
                Some(player) => {
                    player.nodes.push(node_id.clone());
                }
                None => panic!("WAAAAH")
            }
        }
        {
            let rows = gd.map.iter_mut();
            let mut node_option = None;
            for row in rows {
                node_option = row.iter_mut().find(|n| n.id == node_id);
                match node_option {
                    Some(node) => {
                        node_option = Some(node);
                        break;
                    }
                    None       => ()
                }
            }
            match node_option {
                Some(node) => node.owner = player_id.clone(),
                None       => panic!("WAAAAH")
            }
            
        }

        for id in exec_ids {
            gd.add_exec_to_player(&id, &player_id);
        }
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

    // #[test]
    // fn test_find_unemployed_execs_id() {
    //     let dimensions = vec![16,16];
    //     let mut game_data = objects::game_data::GameData::new(&dimensions);

    //     let mut exec = &mut game_data.execs[0];
    //     let mut player = &mut game_data.players[0];

    //     add_exec_to_player(&mut exec, &mut player);

    //     let unemployed = game::find_unemployed_execs_id(&game_data);            
        
    //     assert!(unemployed.len() > 0);
    //     assert_eq!(unemployed.len(), game_data.execs.len()-1);

    //     let mut ids = Vec::new();

    //     for nr in 0..game_data.execs.len() {
    //         let mut s = String::from("E");
    //         s.push_str(&nr.to_string());
    //         ids.push(s);
    //     }

    //     let exec_ids = unemployed.iter();
        
    //     for id in exec_ids {
    //         assert!(ids.contains(&id));
    //     }
    // }
}