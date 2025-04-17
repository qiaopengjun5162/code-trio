// code-trio/packages/go/linked_list/list.go
package linked_list

type ListNode struct {
	Val  int
	Next *ListNode
}

func RemoveNthFromEnd(head *ListNode, k int) *ListNode {
	if head == nil || k <= 0 {
		return head
	}
	dummy := &ListNode{0, head}
	fast, slow := dummy, dummy

	// Fast pointer moves k steps
	for i := 0; i < k; i++ {
		if fast.Next == nil {
			return head
		}
		fast = fast.Next
	}

	// Move fast and slow together
	for fast.Next != nil {
		fast = fast.Next
		slow = slow.Next
	}

	// Remove the kth node from the end
	slow.Next = slow.Next.Next
	return dummy.Next
}

// Helper: Create a linked list from a slice
func CreateLinkedList(values []int) *ListNode {
	if len(values) == 0 {
		return nil
	}
	head := &ListNode{Val: values[0]}
	current := head
	for _, val := range values[1:] {
		current.Next = &ListNode{Val: val}
		current = current.Next
	}
	return head
}

// Helper: Convert linked list to slice
func LinkedListToSlice(head *ListNode) []int {
	result := []int{}
	for head != nil {
		result = append(result, head.Val)
		head = head.Next
	}
	return result
}
