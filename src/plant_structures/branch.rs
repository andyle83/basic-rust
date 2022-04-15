use crate::plant_structures::leave::Leave;

pub struct Branch {
    pub leaves: Vec<Leave>
}

pub fn add_leaves(branch: &mut Branch, leave: Leave) {
    branch.leaves.push(leave);
}