use crate::plant_structures::branch::Branch;
use crate::plant_structures::leave::Leave;

pub struct Root {
    pub node: Branch
}

pub fn init_root() -> Root {
    let l1 = Leave {
        color: "blue".to_string(),
    };

    let l2 = Leave {
        color: "yellow".to_string(),
    };

    let b = Branch {
        leaves: Vec::new(),
    };

    // invoke fn from branch module to add leaves l1 and l2
}