use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Todo {
    pub id: u32,
    pub description: String,
    pub fait: bool,
}

impl Todo {
    pub fn nouveau(id: u32, description: String) -> Todo {
        Todo { id, description, fait: false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nouveau() {
        let t = Todo::nouveau(1, String::from("test"));
        assert_eq!(t.id, 1);
        assert_eq!(t.description, "test");
        assert!(!t.fait);
    }
}
