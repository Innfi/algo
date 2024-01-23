package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

/*
only leaf node can have data value, any node above leaf have keys for index
each node can have multiple keys, limited by the decree
when the number of keys reach the decree, divide keys into two subnodes
each nodes on the same level are sorted, and connected as single linked list

TODO
--------------------------------------------------------------------------------
define node structure
insert, update, delete node while maintaining constraints
assert the search process

DONE
--------------------------------------------------------------------------------

*/

func TestNodeCanHaveKeyValue(t *testing.T) {
	node := NewTreeNode()
	node.Insert(KVSet{
		key:   1,
		value: "dummy",
	})
	assert.Equal(t, node.kvSet[0].key, 1)
}

func TestNodeLinkAsParentChildren(t *testing.T) {
	parent := NewTreeNode()
	parent.Insert(KVSet{
		key:   1,
		value: "dummy",
	})

	child := NewTreeNode()
	child.Insert(KVSet{
		key:   3,
		value: "dummy2",
	})

	parent.children = child
	child.parent = parent

	assert.Equal(t, parent.children.kvSet[0].key, 3)
	assert.Equal(t, child.parent.kvSet[0].key, 1)
}

// func TestNodeLinkToNext(t *testing.T) {
// 	node := TreeNode{
// 		key:   1,
// 		value: "dummy1",
// 	}
//
// 	node.next = &TreeNode{
// 		key:   2,
// 		value: "",
// 	}
//
// 	assert.Equal(t, node.next.key, 2)
// }
//
// func TestNodeMultipleKV(t *testing.T) {
//
// }

func TestInit(t *testing.T) {
	assert.Equal(t, 1, 1)
}
