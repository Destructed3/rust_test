use crate::objects::*;
use std::collections::HashMap;

pub fn add_action(e: &mut exec::Exec, data: HashMap<String, action::ActionData>) {
    let (player_id, exec_id) = (&p.id, &e.id);
    let mut id = "";
    let action = action::Action::new(id, player_id, exec_id, data);
}

#[cfg(test)]
mod tests {
    use crate::objects::*;
    use super::*;

    #[test]
    fn t_add_action() {
        use std::collections::HashMap;
        use crate::objects::action::*;
        let mut e = exec::Exec::new(String::from("Test"), String::from("e1"));

        let mut data: HashMap<String, action::ActionData> = HashMap::new();
        data.insert(String::from("type"), ActionData::Type(ActionType::Buy));

        add_action(&mut e, data);

        assert_eq!(p.actions.len(), 1_usize);
    }

    #[test]
    fn save_game() {

    }

}