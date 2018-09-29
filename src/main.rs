#![feature(vec_remove_item)]

extern crate cybercity;

use cybercity::*;

fn main() {
    let dimensions = vec![16,16];
    let game_data = objects::game_data::GameData::new(&dimensions);

    game::run(game_data);
}
