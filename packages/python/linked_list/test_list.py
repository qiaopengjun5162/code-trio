# code-trio/packages/python/linked_list/test_list.py
import pytest
from .list import ListNode, remove_nth_from_end, create_linked_list, linked_list_to_list

@pytest.fixture
def setup_linked_list():
    """创建测试用链表"""
    def _create(values):
        return create_linked_list(values)
    return _create

def test_remove_nth_from_end_standard(setup_linked_list):
    """测试常规情况：删除倒数第 k 个节点"""
    head = setup_linked_list([1, 2, 6, 3, 4, 6])
    result = remove_nth_from_end(head, 2)
    assert linked_list_to_list(result) == [1, 2, 6, 3, 6]

def test_remove_nth_from_end_single_node(setup_linked_list):
    """测试只有一个节点的链表"""
    head = setup_linked_list([1])
    result = remove_nth_from_end(head, 1)
    assert linked_list_to_list(result) == []

def test_remove_nth_from_end_head(setup_linked_list):
    """测试删除头节点（k 等于链表长度）"""
    head = setup_linked_list([1, 2, 3])
    result = remove_nth_from_end(head, 3)
    assert linked_list_to_list(result) == [2, 3]

def test_remove_nth_from_end_empty():
    """测试空链表"""
    head = None
    result = remove_nth_from_end(head, 1)
    assert linked_list_to_list(result) == []

def test_remove_nth_from_end_last_node(setup_linked_list):
    """测试删除最后一个节点"""
    head = setup_linked_list([1, 2, 3])
    result = remove_nth_from_end(head, 1)
    assert linked_list_to_list(result) == [1, 2]

def test_remove_nth_from_end_invalid_k(setup_linked_list):
    """测试 k 无效（k 比链表长）"""
    head = setup_linked_list([1, 2, 3])
    result = remove_nth_from_end(head, 5)  # k=5 比链表长度大
    # 验证返回的是原链表
    assert result.val == 1
    assert result.next.val == 2
    assert result.next.next.val == 3

def test_remove_nth_from_end():
    # 空链表
    assert remove_nth_from_end(None, 1) is None
    
    # 链表长度 < k
    head = ListNode(1)
    assert remove_nth_from_end(head, 2) == head
    
    # 正常情况
    head = ListNode(1, ListNode(2, ListNode(3)))
    result = remove_nth_from_end(head, 2)
    assert result.val == 1
    assert result.next.val == 3

def test_create_empty_linked_list():
    """测试创建空链表"""
    head = create_linked_list([])
    assert head is None  # 空输入应返回 None

# 测试 create_linked_list 的所有分支
def test_create_linked_list():
    # 空列表
    assert create_linked_list([]) is None
    
    # 单元素链表
    head = create_linked_list([1])
    assert head.val == 1
    assert head.next is None
    
    # 多元素链表
    head = create_linked_list([1, 2, 3])
    assert linked_list_to_list(head) == [1, 2, 3]
