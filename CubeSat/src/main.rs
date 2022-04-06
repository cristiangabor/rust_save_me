use std::ops::{Add};


#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Event {
    message: Vec<Message>,
}

type Message = String;

#[derive(Debug)]
struct TreeCount {
    counter_id: u64,
    pub event: Event
}

fn check_status(counter: TreeCount) -> TreeCount {
    println!("{:?}: {:?}", counter, StatusMessage::Ok);
    
    counter
}

fn add_with_lifetimes<'a, 'b>(i: &'a TreeCount, j: &'b TreeCount) -> u64 {
    i.counter_id
}

fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

struct Orchestrator;


fn send(to: &mut TreeCount, msg: Message) {
    to.event.message.push(msg);
}

fn main() {
    // Each satellite variable is represented by an integer.
    Orchestrator {};

    let tree_count_a = TreeCount { counter_id: 0, event: Event { message: vec![]}};
    let tree_count_b = TreeCount { counter_id: 1, event: Event { message: vec![]}};
    let tree_count_c = TreeCount { counter_id: 2, event: Event { message: vec![]}};

    let tree_count_a = check_status(tree_count_a);
    let tree_count_b = check_status(tree_count_b);
    let tree_count_c = check_status(tree_count_c);

    println!("a: {:?}, b: {:?}, c: {:?}", tree_count_a, tree_count_c, tree_count_c);
    // waiting....

    let tree_count_a = check_status(tree_count_a);
    let tree_count_b = check_status(tree_count_b);
    let tree_count_c = check_status(tree_count_c);


    let res =  add_with_lifetimes(&tree_count_a, &tree_count_b);

    

    println!("a: {:?}, b: {:?}, c: {:?}", tree_count_a, tree_count_b, tree_count_c);    

    let floats = add(1.2, 3.4);

    println!("{:?}, {}", res, floats);
}