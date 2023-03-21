package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

/*
TOOD
--------------------------------------------------------------------------------
- create node and link
- create tree
- insert initial node
- insert nodes
- remove node(s)

DONE
--------------------------------------------------------------------------------
*/

type Node struct {
	nodeValue int
	next      *Node
}

func TestInitNode(t *testing.T) {
	instance := Node{
		nodeValue: 1,
		next:      nil,
	}

	assert.Equal(t, instance.nodeValue, 1)
	assert.Equal(t, instance.next == nil, true)
}
