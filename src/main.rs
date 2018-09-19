extern crate cybercity;

fn main() {
    let dimensions = vec![16,16];
    let logic = cybercity::GameLogic::new(&dimensions);

    let map = logic.map.iter();
    for row in map {
        let nodes = row.iter();
        for node in nodes {
            println!("node at {}/{}, ID: {}", &node.x, &node.y, &node.id);
        }
    }

    let players = logic.players.iter();
    for player in players {
        println!("Player: {}; ID: {}", player.name, player.id);
    }

    let execs = logic.execs.iter();
    for exec in execs {
        println!("Exec: {}; ID: {}", exec.name, exec.id);
    }
}
