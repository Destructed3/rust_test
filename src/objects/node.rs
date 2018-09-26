pub struct Node {
    pub id: String,
    pub x: u32,
    pub y: u32,
    pub owner: String,
}
impl Node {
    pub fn new(coordinates: &[u32]) -> Node {
        let x = coordinates[0].clone();
        let y = coordinates[1].clone();
        let mut id = String::from("N");
        id.push_str(&x.to_string());
        id.push_str(&y.to_string());
        let owner = String::from("");

        Node { id, x, y, owner }
    }

    pub fn change_owner(&mut self, owner_id: &String) {
        self.owner = owner_id.to_owned();
    }
}

mod tests {
    use super::*;

    #[test]
    fn change_owner() {
        let owner = String::from("new_owner");
        let mut node = Node::new(&vec![1,1]);
        node.change_owner(&owner);
        assert_eq!(node.owner, owner);
    }
}