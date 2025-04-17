# code-trio/packages/python/linked_list/list.py
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def remove_nth_from_end(head: ListNode, k: int) -> ListNode:
    # 边界检查：空链表或 k <= 0 直接返回
    if not head or k <= 0:
        return head
    
    # 创建虚拟头节点，简化边界处理
    dummy = ListNode(0, head)
    fast = slow = dummy
    
    # fast 先走 k 步
    for _ in range(k):
        if not fast.next:  # 如果链表长度 < k，直接返回原链表
            return head
        fast = fast.next
    
    # fast 和 slow 同时移动，直到 fast 到达末尾
    while fast.next:
        fast = fast.next
        slow = slow.next
    
    # 删除 slow 指向的下一个节点
    slow.next = slow.next.next
    return dummy.next

# 辅助函数：创建链表
def create_linked_list(values):
    if not values:
        return None
    head = ListNode(values[0])
    current = head
    for val in values[1:]:
        current.next = ListNode(val)
        current = current.next
    return head

# 辅助函数：链表转列表
def linked_list_to_list(head):
    result = []
    while head:
        result.append(head.val)
        head = head.next
    return result
