#![feature(vec_remove_item)]

use crate::objects::player::*;
use crate::objects::exec::*;
use crate::objects::node::*;
use crate::objects::game_data::*;
use crate::objects::action::*;

pub mod objects;

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

}


mod generators {
    extern crate rand;
    extern crate heck;

    use rand::prelude::*;

    pub fn generate_name() -> String {
        use heck::CamelCase;
        
        let mut rng = thread_rng();
        let mut name = String::from("");

        for _ in 0..rng.gen_range(1,4) {
            name.push_str(add_syllable());
        }
        if name.len() < 2 {
            name.push_str(add_syllable());
        }

        name.to_camel_case()
    }
    fn add_syllable() -> &'static str {
        let mut rng = thread_rng();
        let syls = vec!["al", "ham", "bra", "a", "chil", "les", "o", "din", "heim", "dal"];
        let syl = rng.gen_range(0, syls.len()-1);

        syls[syl]
    }

    #[cfg(test)]
    mod tests {
        use super::*;   

        #[test]
        fn namegeneration() {
            use crate::objects::player;
            let obj = player::Player::new(String::from("1"), generate_name());
            assert!(obj.name.len() > 1);
        }        
    }
}

mod tests {
    use super::*;
    
    #[test]
    fn setup_players() {
        let dimensions = vec![16,16];
        let game_data = crate::objects::game_data::GameData::new(&dimensions);
        
        crate::setup_players(&game_data);

        let _: Vec<_> = game_data.players.iter().map( |player| {
            assert_eq!(player.execs.len(), 2);
            assert_eq!(player.nodes.len(), 1);
        }).collect();
    }
}
