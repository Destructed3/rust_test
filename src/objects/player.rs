use super::*;

pub struct Player<'a> {
    pub id: String,
    pub name: String,
    pub money: u32,
    pub nodes: Vec<&'a mut node::Node>,
    pub execs: Vec<exec::Exec>,
}

impl<'a> Player<'a> {
    pub fn new(id: String, name: String, money: u32) -> Player<'a> {
        let nodes: Vec<&'a mut node::Node> = Vec::new();
        let execs: Vec<exec::Exec> = Vec::new();

        Player { id, name, money, nodes, execs }
    }

    pub fn add_node(&mut self, node: &'a mut node::Node) {
        self.nodes.push(node);
    }
    pub fn remove_node(&mut self, node_id: &str) -> bool {
        let i_opt = self.nodes.iter().position( |n| n.id == node_id );
        match i_opt {
            Some(i) => {
                self.nodes.remove(i);
                true
            },
            None => false
        }
    }
    pub fn remove_exec(&mut self, exec_id: &str) -> bool {
        let i_opt = self.execs.iter().position( |e| e.id == exec_id );
        match i_opt {
            Some(i) => {
                self.execs.remove(i);
                true
            },
            None => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_node() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"),0);
        let node = node::Node::new(&[1,1]);
        let id = node.id.to_string();
        obj.nodes.push(&mut node);
        assert_eq!(obj.nodes.len(), 1);
        assert!( obj.remove_node(&id) );
        assert_eq!(obj.nodes.len(), 0);
        assert!( !obj.remove_node(&id));
    }

    #[test]
    fn remove_exec() {
        let mut obj = Player::new(String::from("1"), String::from("Egon"),0);
        let exec = exec::Exec::new("ID", "NAME");
        obj.execs.push(exec);
        assert_eq!(obj.execs.len(), 1);
        assert!(obj.remove_exec("ID"));
        assert_eq!(obj.execs.len(), 0);
        assert!( !obj.remove_exec("ID") );
    }
}
