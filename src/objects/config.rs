pub struct Config {
    pub map_size: Vec<u32>,
    pub player_number: u32,
    pub starting_nodes: u32,
    pub starting_distance: u32,
    pub starting_boarder_distance: u32,
    pub starting_execs: u32
}

impl Config {
    pub fn new(map_size: Vec<u32>, player_number: u32, starting_nodes: u32, starting_distance: u32, starting_boarder_distance: u32) -> Config {
        let starting_execs = player_number * 3;
        Config { 
            map_size, 
            player_number, 
            starting_nodes, 
            starting_distance, 
            starting_boarder_distance, 
            starting_execs 
        }
    }

    pub fn read_config() -> Config {
        let dimensions = vec![16,16];
        Config::new(dimensions, 4, 1, 2, 2)
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
}