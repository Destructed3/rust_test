#![feature(vec_remove_item)]

use crate::objects::player::*;
use crate::objects::exec::*;
use crate::objects::node::*;
use crate::objects::game_data::*;

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

mod generators {
    extern crate rand;
    extern crate heck;

    use rand::prelude::*;

    pub fn generate_name() -> String {
        use heck::CamelCase;
        let mut rng = thread_rng();
        let syls = vec!["al", "ham", "bra", "a", "chil", "les", "o", "din", "heim", "dal"];
        let mut name = String::from("");
        for _ in 0..rng.gen_range(1,4) {
            let syl = rng.gen_range(0, syls.len()-1);
            name.push_str(syls[syl]);
        }

        name.to_camel_case()
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


