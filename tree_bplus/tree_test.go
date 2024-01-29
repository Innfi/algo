package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

/*
only leaf node can have data value, any node above leaf have keys for index
each node can have multiple keys, limited by the decree
when the number of keys reach the limit, divide keys into two subnodes
each nodes on the same level are sorted, and connected as single linked list

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
* insert, update, delete node while maintaining constraints
  - insert: before the limit
* assert the search process

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
