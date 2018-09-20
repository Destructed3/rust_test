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
    let players = gd.players.iter();
    for player in players {
        
    }
}

fn find_unemployed_execs_id(gd: &GameData) -> Vec<String> {
    let mut unemployed: Vec<String> = Vec::new();

    let execs = gd.execs.iter();
    for exec in execs {
        if exec.employer == "" {
            unemployed.push(exec.id.clone());
        }
    }

    unemployed
}

mod tests {
    use super::*;
    
    #[test]
    fn setup_players() {
        let dimensions = vec![16,16];
        let game_data = objects::game_data::GameData::new(&dimensions);
        
        game::setup_players(&game_data);

        let _players: Vec<_> = game_data.players.iter().map( |player| {
            assert_eq!(player.execs.len(), 2);
            assert_eq!(player.nodes.len(), 1);
        }).collect();
    }

    #[test]
    fn find_unemployed_execs_id() {
        let dimensions = vec![16,16];
        let game_data = objects::game_data::GameData::new(&dimensions);

        let unemployed = game::find_unemployed_execs_id(&game_data);            
        
        assert!(unemployed.len() > 0);

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
    fn add_exec_to_player() {
        let dimensions = vec![16,16];
        let gd = objects::game_data::GameData::new(&dimensions);

        let exec_id = gd.execs[0].id.clone();
        let player_id = gd.players[0].id.clone();

        add_exec_to_player(exec_id.clone(), player_id.clone());
    }
}