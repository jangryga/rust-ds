pub mod linked_list;
pub mod binary_tree;

#[cfg(test)]
mod tests {
    use crate::binary_tree::TreeNode;
    use crate::linked_list::LinkedList;
    use crate::linked_list; 

    #[test]
    fn create_linked_list() {
        let ls1 = linked_list!([1, 2, 3]);
        let mut ls2 = LinkedList::new();
        ls2.add(1);
        ls2.add(2);
        ls2.add(3);
        assert_eq!(ls1, ls2);
    }

    #[test]
    fn use_linked_list_macro() {
        let mut ls1 = linked_list!();
        let ls2 = linked_list!(1);
        let ls3 = linked_list!([1]);
        ls1.add(1);
        assert_eq!(ls1, ls2);
        assert_eq!(ls1, ls3);
    }

    #[test]
    fn create_binary_tree() {
        let tree = TreeNode::new(2);
        assert!(true);
    }
}
