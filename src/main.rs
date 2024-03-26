//use bloomfilter::Bf;
use uuid::Uuid;
pub mod bloomfilter;
pub mod kvstore;
pub mod binaryheap;
pub mod models {
    pub mod informationcriterion;
    mod common;
}
/*
Maybe for each key the value is a nested hashmap/list with a bloomfilter for each value or and for each table
 */

fn main() {
    // let mut table1 = kvstore::Kv::new("table1");

    // match table1.put(1, "a") {
    //     Ok(_) => println!("Put Successful"),
    //     Err(err) => println!("Err: {}", err),
    // }

    // match table1.get(2) {
    //     Ok(value) => println!("Got: {}", *value),
    //     Err(err) => println!("Err: {}", err),
    // }

    let mut bloom1: bloomfilter::Bf = bloomfilter::Bf::new("bloom1");

    bloom1.printer();

    bloom1.set("HelloWorld");

    // let bf_true = bloom1.look_up("HelloWorld");
    // println!("Test bf look_up true: {}", bf_true);

    // let bf_false = bloom1.look_up("HelloWorld");
    // println!("Test bf look_up false: {}", bf_false);

    // let mut bloom2 = bloomfilter::Bf::new("bloom2");
    // bloom2.printer();
    // bloom2.set("Hello Moon");

    // let bf_true = bloom2.look_up("Hello Moon");
    // println!("Test bf look_up true: {}", bf_true);

    // let bf_false = bloom2.look_up("Hello Moon 3434");
    // println!("Test bf look_up false: {}", bf_false);

    // let mut x = true;
    // while x {
    //     //println!("{}", i);
    //     let id1 = Uuid::new_v4();
    //     let id2 = Uuid::new_v4();
    //     x = bloom1.set(&id1.to_string());
    //     if bloom1.look_up(&id2.to_string()) {
    //         println!("{}, {}", id1, id2);
    //         println!(
    //             "num elem: {}, len bit_ven: {}",
    //             bloom1.num_elements,
    //             bloom1.bit_vec.len()
    //         );
    //         println!("Broken");
    //         //println!("{:?}", bloom1.bit_vec);
    //         x = false;
    //     }
    // }

    let mut bheap: binaryheap::BinaryHeap<&str> = binaryheap::BinaryHeap::new();

}
