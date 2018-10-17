#![feature(vec_remove_item)]

extern crate cybercity;

use cybercity::*;
use self::objects::{config, game_data};

fn main() {
    let config = config::Config::read_config();
    let game_data = game_data::GameData::new(config);

    game::run(game_data);
}
