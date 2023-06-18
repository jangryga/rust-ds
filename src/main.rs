
use data_structures::linked_list;


fn main() {
    let mut ls = linked_list!([1, 2, 3]);

    ls.add(4);
    ls.add(5);

    println!("{:?}", ls);
}
