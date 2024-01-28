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
insert, update, delete node while maintaining constraints
  - 1st step: if the number of kvset reaches the threshold (or the decree),
	            children whill have divided subset of the kvset
							node will contain key set only
assert the search process

DONE
--------------------------------------------------------------------------------
define node structure
  - node contains links to children, and next node
	- node contains multiple key / value set (as arrays?)

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

	assert.Equal(t, parent.children.kvSet[0].key, 3)
}

func TestNodeLinkToNext(t *testing.T) {
	root := NewTreeNode()
	root.Insert(KVSet{
		key:   1,
		value: "dummy1",
	})

	root.next = NewTreeNode()
	root.next.Insert(KVSet{
		key:   2,
		value: "",
	})

	assert.Equal(t, root.next.kvSet[0].key, 2)
}

func TestNodeMultipleKV(t *testing.T) {
	root := NewTreeNode()
	root.Insert(KVSet{
		key:   1,
		value: "value1",
	})
	root.Insert(KVSet{
		key:   2,
		value: "value2",
	})

	assert.Equal(t, len(root.kvSet), 2)
}
