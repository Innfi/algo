package main

import (
	"testing"

	"github.com/stretchr/testify/assert"

	ref "algo/tree-bplus/src"
)

/*
* only leaf node can have data value, any node above leaf have keys for index
* each node can have multiple keys, limited by the degree
* when the number of keys reach the limit, divide keys into two subnodes
* each nodes on the same level are sorted, and connected as single linked list

TODO
--------------------------------------------------------------------------------
* define node structure
  - node structure consists of three element:
	  pointer to parent node
		pointer to next node
		key / value set
	- each value of key / value set can contain only one of:
	  pointer to child node, or
		actual value
* utility functions
	- split the kvset
	- check the identity of a node:
	  - if kvset contains actual values, then it is a leaf node
		- if kvset contains pointers to children, then it is a middle node
* search node
* insert, update, delete node while maintaining constraints
  - insert: before the limit

DONE
--------------------------------------------------------------------------------

*/

func TestArrayPreset(t *testing.T) {
	input := make([]KVList, 5)

	assert.Equal(t, len(input), 5)
}

func TestNodeAddValueBeforeLimit(t *testing.T) {
	node := Node{
		kvList: make([]KVList, 0, 5),
	}

	node.kvList = append(node.kvList, KVList{
		key:  1,
		data: "initial",
	})

	assert.Equal(t, len(node.kvList), 1)
	assert.Equal(t, node.kvList[0].data, "initial")
}

func TestSplitKVList(t *testing.T) {
	node := CreateNode()

	// not implementing node.Append()
	// as splitting kvlist is part of the append process
	node.kvList = append(node.kvList, KVList{
		key:  1,
		data: "first",
	})
	node.kvList = append(node.kvList, KVList{
		key:  2,
		data: "second",
	})
	node.kvList = append(node.kvList, KVList{
		key:  3,
		data: "third",
	})
	node.kvList = append(node.kvList, KVList{
		key:  4,
		data: "fourth",
	})
	node.kvList = append(node.kvList, KVList{
		key:  5,
		data: "fifth",
	})

	SplitKVList(node)

	// after the node split, kvlist should be divided in half
	assert.Equal(t, len(node.kvList), 2)
	assert.Equal(t, node.kvList[0].data, "")
	assert.Equal(t, node.kvList[1].data, "")

	assert.Equal(t, node.kvList[0].child != nil, true)
	assert.Equal(t, node.kvList[1].child != nil, true)

	// testing leaf node
	firstChild := node.kvList[0].child
	assert.Equal(t, len(firstChild.kvList), 2)
	assert.Equal(t, firstChild.kvList[0].key, 1)
	assert.Equal(t, firstChild.kvList[0].data, "first")

	secondChild := node.kvList[1].child
	assert.Equal(t, len(secondChild.kvList), 3)
	assert.Equal(t, secondChild.kvList[1].key, 4)
}

// func TestSplitKVListOrder(t *testing.T) {
// 	// keys should be sorted regardless of input order
// }

func TestReference(t *testing.T) {
	instance := ref.NewBTree()

	instance.Insert(1)
	instance.Insert(5)
	instance.Insert(10)
	instance.Insert(7)

	root := instance.Root()
	keys := root.Keys()
	pointers := root.Pointers()

	assert.Equal(t, len(keys), 1)
	assert.Equal(t, len(pointers), 2)

	assert.Equal(t, keys[0], 7)

	firstChild := pointers[0]
	firstKeys := firstChild.Keys()
	firstPointers := firstChild.Pointers()

	assert.Equal(t, len(firstKeys), 2)
	assert.Equal(t, len(firstPointers), 0)
	assert.Equal(t, firstKeys[0], 1)
	assert.Equal(t, firstKeys[1], 5)

	nextChild := firstChild.Next()
	assert.Equal(t, nextChild, pointers[1])
}
