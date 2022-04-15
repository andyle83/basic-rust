use super::branch::{Branch, add_leaves};
use super::leave::Leave;

pub struct Root {
    pub node: Branch
}

#[test]
pub fn init_root() {
    let l1 = Leave {
        color: "blue".to_string(),
    };

    let l2 = Leave {
        color: "yellow".to_string(),
    };

    let b = &mut Branch {
        leaves: Vec::new(),
    };

    // invoke fn from branch module to add leaves l1 and l2
    add_leaves(b, l1);
    add_leaves(b, l2);

    assert_eq!(b.leaves.len(), 2);
}