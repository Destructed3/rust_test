#[cfg(test)]
mod tests {
    #[test]
    fn do_action() {
        
    }

    #[test]
    fn create_queue() {
        // aka collect actions
        use crate::objects::{exec, player};
        let mut player = player::Player::new("id".to_string(), "Fred".to_string(), 1);
    }

    #[test]
    fn run_queue() {

    }

    #[test]
    fn reset_queue() {

    }
}