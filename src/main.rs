
use data_structures::linked_list::LinkedList;

fn main() {
    let mut ls: LinkedList = LinkedList::new();

    ls.add(1);
    ls.add(2);
    ls.add(3);
    ls.add(4);
    ls.add(5);

    println!("{:?}", ls);
}
