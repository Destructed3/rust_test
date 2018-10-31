use super::*;

mod startup;
mod endturn;
// mod actions;

/// Main function to start off the game
pub fn run(game_data: GameData) {
    let gd = startup::setup_players(game_data);
    let map = gd.map.iter();
    for row in map {
        let nodes = row.iter();
        for node in nodes {
            println!("node at {}/{}, ID: {}, Owner: {}", &node.x, &node.y, &node.id, &node.owner);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_players() {
        let mut game_data = GameData::new(Config::read_config());
        
        game_data = startup::setup_players(game_data);

        let _players: Vec<_> = game_data.players.iter().map( |player| {
            assert_eq!(player.execs.len(), 2);
            assert_eq!(player.nodes.len(), 1);
        }).collect();
    }

}
