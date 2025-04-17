// 定义链表节点结构体
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || k <= 0 {
        return head;
    }
    let mut dummy = Box::new(ListNode { val: 0, next: head });

    let mut fast = &mut dummy.clone();
    let mut slow = &mut dummy;

    for _ in 0..k {
        if fast.next.is_none() {
            return dummy.next;
        }
        fast = fast.next.as_mut().unwrap();
    }

    while fast.next.is_some() {
        fast = fast.next.as_mut().unwrap();
        slow = slow.next.as_mut().unwrap();
    }

    slow.next = slow.next.take().unwrap().next.clone();

    dummy.next
}

pub fn remove_nth_from_end_unsafe(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // 处理空链表或 n <= 0
    if head.is_none() || n <= 0 {
        return head;
    }
    let mut dummy = Box::new(ListNode { val: 0, next: head });

    unsafe {
        let mut slow = &mut dummy as *mut Box<ListNode>;
        let mut fast = &mut dummy as *mut Box<ListNode>;

        for _ in 0..n {
            if (*fast).next.is_none() {
                return dummy.next;
            }
            fast = (*fast).next.as_mut().unwrap();
        }

        while (*fast).next.is_some() {
            fast = (*fast).next.as_mut().unwrap();
            slow = (*slow).next.as_mut().unwrap();
        }

        (*slow).next = (*slow).next.take().unwrap().next;
    }

    dummy.next
}

pub fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut curr = &mut dummy;

    for &val in vec.iter() {
        curr.next = Some(Box::new(ListNode::new(val)));
        curr = curr.next.as_mut().unwrap();
    }

    dummy.next
}

pub fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut curr = list;

    while let Some(node) = curr {
        vec.push(node.val);
        curr = node.next;
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_case() {
        let input = vec_to_list(vec![1, 2, 3, 4, 5]);
        let result = remove_nth_from_end(input, 2);
        assert_eq!(list_to_vec(result), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_delete_head() {
        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end(input, 3);
        assert_eq!(list_to_vec(result), vec![2, 3]);
    }

    #[test]
    fn test_delete_tail() {
        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end(input, 1);
        assert_eq!(list_to_vec(result), vec![1, 2]);
    }

    #[test]
    fn test_single_node() {
        let input = vec_to_list(vec![1]);
        let result = remove_nth_from_end(input, 1);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn test_empty_list() {
        let input = vec_to_list(vec![]);
        let result = remove_nth_from_end(input, 1);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn test_k_greater_than_length() {
        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end(input, 4);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn test_k_zero_or_negative() {
        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end(input, 0);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);

        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end(input, -1);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn test_k_equals_length() {
        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end(input, 3);
        assert_eq!(list_to_vec(result), vec![2, 3]);
    }

    // 测试 remove_nth_from_end_unsafe
    #[test]
    fn test_unsafe_normal_case() {
        let input = vec_to_list(vec![1, 2, 3, 4, 5]);
        let result = remove_nth_from_end_unsafe(input, 2);
        assert_eq!(list_to_vec(result), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_unsafe_delete_head() {
        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end_unsafe(input, 3);
        assert_eq!(list_to_vec(result), vec![2, 3]);
    }

    #[test]
    fn test_unsafe_delete_tail() {
        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end_unsafe(input, 1);
        assert_eq!(list_to_vec(result), vec![1, 2]);
    }

    #[test]
    fn test_unsafe_single_node() {
        let input = vec_to_list(vec![1]);
        let result = remove_nth_from_end_unsafe(input, 1);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn test_unsafe_empty_list() {
        let input = vec_to_list(vec![]);
        let result = remove_nth_from_end_unsafe(input, 1);
        assert_eq!(list_to_vec(result), vec![]);
    }

    #[test]
    fn test_unsafe_k_greater_than_length() {
        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end_unsafe(input, 4);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn test_unsafe_k_zero_or_negative() {
        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end_unsafe(input, 0);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);

        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end_unsafe(input, -1);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn test_unsafe_k_much_larger_than_length() {
        let input = vec_to_list(vec![1, 2, 3]);
        let result = remove_nth_from_end_unsafe(input, 100);
        assert_eq!(list_to_vec(result), vec![1, 2, 3]);
    }

    #[test]
    fn test_unsafe_short_list_edge_case() {
        let input = vec_to_list(vec![1]);
        let result = remove_nth_from_end_unsafe(input, 2);
        assert_eq!(list_to_vec(result), vec![1]);
    }
}
