use super::*;

/// Main function to start off the game
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

/// distributes starting ressources to players
fn setup_players(mut gd: GameData) -> GameData {
    for i in 0..gd.players.len() {
        let player_id = gd.players[i].id.clone(); 

        let exec_ids = find_start_execs(&gd);
        for id in exec_ids {
            gd.add_exec_to_player(&id, &player_id);
        }
        
        let node_id = find_start_node(&gd).expect("{}");
        gd.add_node_to_player(&node_id, &player_id);
    }
    gd
}

fn find_start_node(gd: &GameData) -> Result<String, String> {
    let mut node_id: &str = "";
    let rows = gd.map.iter();
    for row in rows {
        let nodes = row.iter();
        for node in nodes {
            if node_allowed(&gd, &node) {
                node_id = &node.id;
                break;
            }
        }
        if node_id.len() > 0 {
            break;
        }
    }
    if node_id == "" {
        return Err(String::from("Fuck"));
    }
    return Ok(node_id.to_owned())    
}

fn find_start_execs(gd: &GameData) -> Vec<String> {
    // Make this loop variable? -> config would be part of GameData
    let mut exec_ids: Vec<String> = Vec::with_capacity(2);
    while exec_ids.len() < exec_ids.capacity() {
        let execs = gd.execs.iter();
        for exec in execs {
            if exec.employer == "" && !exec_ids.contains(&exec.id) {
                exec_ids.push(exec.id.to_owned());
                break;
            }
        }
    }

    exec_ids
}

fn node_allowed(gd: &GameData, node: &Node) -> bool {
    // Ownerless
    if node.owner != "" {
        return false;
    }
    // Away from map-boarders
    if node.y < 2 || node.y > gd.map.len() as u32 -3 {
        return false;
    }
    if node.x < 2 || node.x > gd.map[0].len() as u32 -3 {
        return false;
    }
    let check_blocker = |old_node: &Node| {
        old_node.owner != "" && (node.x < old_node.x+3) && (node.y < old_node.y+3)
    };
    for row in gd.map.iter() {
        if row.iter().filter( |n| check_blocker(n) ).count() > 0 {
            return false;
        }
    }
    true
}

mod tests {
    #[allow(unused_imports)]
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

    #[test]
    fn test_find_start_node() {
        use std::collections::HashMap;
        // Config
        let nodes_per_length = |length: f32, space_to_boarder: f32, space_between: f32| { return ((length - 2_f32*space_to_boarder) / (space_between+1_f32)).ceil() };
        let (map_len, map_hei, space_to_boarder, space_between) = (8, 8, 2, 2); // To be moved to config-struct
        let nodes_per_row = nodes_per_length(map_len as f32, space_to_boarder as f32, space_between as f32) as usize;
        let nodes_per_col = nodes_per_length(map_hei as f32, space_to_boarder as f32, space_between as f32) as usize;
        let nr_nodes = nodes_per_row * nodes_per_col;

        let mut game_data = objects::game_data::GameData::new(&vec![map_len,map_hei]);
        let player_id = game_data.players[0].id.to_owned();
        let mut node_coord: Vec<HashMap<&str, u32>> = Vec::with_capacity(nr_nodes);
        // Test starts here
        while node_coord.len() < node_coord.capacity() {
            let node_id = find_start_node(&game_data).expect("{}");
            let node = game_data.get_node(&node_id);
            // Ensure real node without owner
            assert_eq!(node.owner, "", "Node nr {}", node_coord.len()+1);
            let hm: HashMap<&str, u32> = [
                ("x", node.x),
                ("y", node.y)
                ].iter().cloned().collect();
            node_coord.push(hm);
            game_data.add_node_to_player(&node_id, &player_id);
        }
        for node in node_coord.iter() {
            let x = node.get("x").unwrap();
            let y = node.get("y").unwrap();
            // Node 2 rows and cols from map-boarders away
            assert!( x > &(1), "X should be > {} -> x: {} | {:?}", space_to_boarder-1, x, node_coord );
            assert!( x < &( map_len - 1), "X should be < {} -> x: {} | {:?}", map_len - 1, x, node_coord);
            assert!( y > &(1), "Y should be > {} -> y: {}, | {:?}", space_to_boarder-1, y, node_coord );
            assert!( y < &(map_hei - 1), "Y should be < {} -> y: {} | {:?}",(map_hei - 1), y, node_coord );
            // Node at least 2 rows and cols from next player-owned node
            let check = |n: &HashMap<&str, u32>, arg: &str| {
                let coord = node.get(arg).unwrap();
                let _coord = n.get(arg).unwrap();
                if _coord < &(space_to_boarder+1)  {
                    return coord > &(space_to_boarder+space_between+2); // +2 because you need to account for 2 nodes
                } else {
                    return coord > &(_coord+space_between+1) && coord < &(_coord-space_between-1);
                }
            };
            let close_nodes_x = node_coord.iter().filter( |n| check(n, "x") ).count();
            let close_nodes_y = node_coord.iter().filter( |n| check(n, "y") ).count();
            assert_eq!( close_nodes_x, 0, "Owned starting nodes to close to each other (x-row) -> {} nodes too close, {} Nodes required; {:?}", close_nodes_x, nr_nodes, node_coord );
            assert_eq!( close_nodes_y, 0, "Owned starting nodes to close to each other (y-row) -> {} nodes too close, {} Nodes required; {:?}", close_nodes_y, nr_nodes, node_coord );
        }
    }

    #[test]
    fn test_find_start_execs() {
        let mut game_data = objects::game_data::GameData::new(&vec![16,16]);
        let exec_ids = find_start_execs(&game_data);
        let execs_ids_iter = exec_ids.iter();
        for exec_id in execs_ids_iter {
            assert_eq!(exec_ids.iter().filter( |&e| e == exec_id ).count(), 1);
            assert_eq!(game_data.get_exec(exec_id).employer, "");
        }
    }
}