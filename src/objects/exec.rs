use super::*;

//#[derive(Debug)]
pub struct Exec {
    pub id: String,
    pub name: String,
    pub employer: String,
    pub action_queue: Vec<action::Action>,
}
impl Exec {
    pub fn new(id: String, name: String) -> Exec {
        let employer = String::from("");
        let action_queue = Vec::new();

        Exec { id, name, employer, action_queue }
    }

    pub fn change_employer(&mut self, employer: &String) {
        self.employer = employer.to_owned();
    }

    pub fn add_action(&mut self, action: action::Action) {
        self.action_queue.push(action);
    }
    pub fn remove_action(&mut self, action_id: &str) {
        let action = self.action_queue.iter().find(|a| a.id == action_id );
        match action {

        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn change_employer() {
        let employer = String::from("P12");
        let mut exec = Exec::new(String::from("1"), String::from("Egon"));
        exec.change_employer(&employer);
        assert_eq!(exec.employer, employer);
    }

    #[test]
    fn add_action() {
        let action = String::from("A12");
        let mut exec = Exec::new(String::from("1"), String::from("Egon"));
        exec.add_action(&action);
        assert_eq!(exec.action_queue[0], action);
    }

    #[test]
    fn remove_action() {
        let action = String::from("A12");
        let mut exec = Exec::new(String::from("1"), String::from("Egon"));
        exec.add_action(&action);
        exec.remove_action(&action);
        assert_eq!(exec.action_queue.len(), 0);
    }
}