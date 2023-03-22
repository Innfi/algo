package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

/*
TOOD
--------------------------------------------------------------------------------
- create tree
- insert initial node
- insert nodes
- remove node(s)

DONE
--------------------------------------------------------------------------------
- create node and link
*/

func TestLink(t *testing.T) {
	rootNode := DoubleNode{
		nodeValue: 1,
		prev:      nil,
		next:      nil,
	}

	secondNode := DoubleNode{
		nodeValue: 2,
		prev:      &rootNode,
		next:      nil,
	}

	rootNode.next = &secondNode

	assert.Equal(t, rootNode.next.nodeValue, 2)
	assert.Equal(t, secondNode.prev.nodeValue, 1)
}

func TestInitNode(t *testing.T) {
	instance := Node{
		nodeValue: 1,
		next:      nil,
	}

	assert.Equal(t, instance.nodeValue, 1)
	assert.Equal(t, instance.next == nil, true)
}

func TestStateChange(t *testing.T) {
	tree := RBTree{}
	tree.root = &DoubleNode{
		nodeValue: 3,
		prev:      nil,
		next:      nil,
	}

	assert.Equal(t, tree.root.nodeValue, 3)
}

func TestInsertFirst(t *testing.T) {
	tree := RBTree{}

	tree.insert(1)

	assert.Equal(t, tree.root.nodeValue, 1)
}
