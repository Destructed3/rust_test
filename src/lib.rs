#![feature(vec_remove_item)]

use crate::objects::player::*;
use crate::objects::exec::*;
use crate::objects::node::*;
use crate::objects::game_data::*;
use crate::objects::action::*;
use crate::objects::config::*;

pub mod objects;
pub mod game;

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
