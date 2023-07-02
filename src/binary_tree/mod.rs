#[derive(Debug)]
pub struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            value: val,
            right: None,
            left: None
        }
    }
}