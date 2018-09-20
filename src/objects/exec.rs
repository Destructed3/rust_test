pub struct Exec {
    pub id: String,
    pub name: String,
    pub employer: String,
}
impl Exec {
    pub fn new(id: String, name: String) -> Exec {
        let employer = String::from("");

        Exec { id, name, employer }
    }

    pub fn change_employer(&mut self, employer: String) {
        self.employer = employer;
    }
}

mod tests {
    use super::*;

    #[test]
    fn change_employer() {
        let employer = String::from("P12");
        let mut exec = Exec::new(String::from("1"), String::from("Egon"));
        exec.change_employer(employer.clone());
        assert_eq!(exec.employer, employer);
    }
}