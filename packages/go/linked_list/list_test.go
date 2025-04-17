// code-trio/packages/go/linked_list/list_test.go
package linked_list

import (
	"reflect"
	"testing"
)

func TestRemoveNthFromEnd(t *testing.T) {
	tests := []struct {
		name     string
		values   []int
		k        int
		expected []int
	}{
		{
			name:     "Standard case: remove k=2 from [1,2,6,3,4,6]",
			values:   []int{1, 2, 6, 3, 4, 6},
			k:        2,
			expected: []int{1, 2, 6, 3, 6},
		},
		{
			name:     "Single node: remove k=1 from [1]",
			values:   []int{1},
			k:        1,
			expected: []int{},
		},
		{
			name:     "Remove head: remove k=3 from [1,2,3]",
			values:   []int{1, 2, 3},
			k:        3,
			expected: []int{2, 3},
		},
		{
			name:     "Empty list: remove k=1 from []",
			values:   []int{},
			k:        1,
			expected: []int{},
		},
		{
			name:     "Remove last node: remove k=1 from [1,2,3]",
			values:   []int{1, 2, 3},
			k:        1,
			expected: []int{1, 2},
		},
		{
			name:     "Invalid k: remove k=4 from [1,2,3]",
			values:   []int{1, 2, 3},
			k:        4,
			expected: []int{1, 2, 3},
		},
		{
			name:     "Invalid k: remove k=0 from [1,2,3]",
			values:   []int{1, 2, 3},
			k:        0,
			expected: []int{1, 2, 3},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			head := CreateLinkedList(tt.values)
			result := RemoveNthFromEnd(head, tt.k)
			got := LinkedListToSlice(result)
			if !reflect.DeepEqual(got, tt.expected) {
				t.Errorf("RemoveNthFromEnd(%v, %d) = %v; want %v", tt.values, tt.k, got, tt.expected)
			}
		})
	}
}
