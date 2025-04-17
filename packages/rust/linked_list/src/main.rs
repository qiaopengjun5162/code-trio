// src/main.rs
use linked_list::{list_to_vec, remove_nth_from_end, vec_to_list};

fn main() {
    let input = vec_to_list(vec![1, 2, 3, 4, 5]);
    let result = remove_nth_from_end(input, 2);
    println!("Output: {:?}", list_to_vec(result)); // 应打印 [1, 2, 3, 5]
}
