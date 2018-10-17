pub struct Config {
    pub map_size: Vec<u32>,
    pub player_number: u32,
    pub starting_money: u32,
    pub starting_nodes: u32,              // per Player
    pub starting_distance: u32,
    pub starting_boarder_distance: u32,
    pub starting_execs: u32               // per Player
}

impl Config {
    pub fn new(map_size: Vec<u32>, player_number: u32, starting_money: u32, starting_nodes: u32, starting_distance: u32, starting_boarder_distance: u32, starting_execs: u32) -> Config {
        Config { 
            map_size, 
            player_number, 
            starting_money,
            starting_nodes,
            starting_distance, 
            starting_boarder_distance, 
            starting_execs 
        }
    }

    /// Will at some point read out a config file
    pub fn read_config() -> Config {
        Config::new(vec![16,16], 4, 1000000, 1, 2, 2, 2)
    }

    pub fn map_size(&self, dim: &str) -> usize {
        if dim == "x" {
            return self.map_size[1] as usize;
        } else if dim == "y" {
            return self.map_size[0] as usize;
        } else {
            panic!();
        }
    }

    pub fn starting_execs(&self) -> u32 {
        self.starting_execs*self.player_number
    }
}