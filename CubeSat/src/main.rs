#![allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct TreeCount {
    counter: u64,
}

fn check_status(counter: TreeCount) -> TreeCount {
    println!("{:?}: {:?}", counter, StatusMessage::Ok);
    
    counter
}

fn main() {
    // Each satellite variable is represented by an integer.
    let tree_count_a = TreeCount { counter: 0 };
    let tree_count_b = TreeCount { counter: 1 };
    let tree_count_c = TreeCount { counter: 2 };

    let tree_count_a = check_status(tree_count_a);
    let tree_count_b = check_status(tree_count_b);
    let tree_count_c = check_status(tree_count_c);

    println!("a: {:?}, b: {:?}, c: {:?}", tree_count_a, tree_count_c, tree_count_c);
    // waiting....

    let tree_count_a = check_status(tree_count_a);
    let tree_count_b = check_status(tree_count_b);
    let tree_count_c = check_status(tree_count_c);

    println!("a: {:?}, b: {:?}, c: {:?}", tree_count_a, tree_count_b, tree_count_c);    
}