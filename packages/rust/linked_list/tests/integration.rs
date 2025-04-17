// tests/integration.rs
use linked_list::{list_to_vec, remove_nth_from_end, vec_to_list};

#[test]
fn test_main_case() {
    let input = vec_to_list(vec![1, 2, 3, 4, 5]);
    let result = remove_nth_from_end(input, 2);
    assert_eq!(list_to_vec(result), vec![1, 2, 3, 5]);
}
