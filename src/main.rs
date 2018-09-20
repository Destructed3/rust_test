#![feature(vec_remove_item)]

extern crate cybercity;

use cybercity::objects::*;

fn main() {
    let dimensions = vec![16,16];
    let game_data = game_data::GameData::new(&dimensions);

    cybercity::game::run(game_data);
}
