package main

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
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

func TestNodeAddValueBeforeLimit(t *testing.T) {
	node := Node{
		kvset: make(map[int]ValueDef),
	}

	node.kvset[1] = ValueDef{
		data: "initial",
	}

	assert.Equal(t, node.kvset[1].data, "initial")
}

func TestSplitKVSet(t *testing.T) {
	node := Node{
		parent: nil,
		next:   nil,
		kvset:  make(map[int]ValueDef),
	}

	node.kvset[1] = ValueDef{
		child: nil,
		data:  "first",
	}
	node.kvset[2] = ValueDef{
		child: nil,
		data:  "second",
	}
	node.kvset[3] = ValueDef{
		child: nil,
		data:  "third",
	}

	// keys are not sorted. what now?
	for index, subset := range node.kvset {
		fmt.Println("index: ", index)
		fmt.Println("subset: ", subset.data)
	}

	assert.Equal(t, len(node.kvset), 3)
}
