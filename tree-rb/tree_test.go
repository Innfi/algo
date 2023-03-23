package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

/*
TOOD
--------------------------------------------------------------------------------
- mark node as red/black according to constraints
- insert nodes
- remove node(s)

DONE
--------------------------------------------------------------------------------
- create node and link
- create tree
- insert initial node
*/

func TestLink(t *testing.T) {
	rootNode := DoubleNode{
		nodeValue: 1,
		left:      nil,
		right:     nil,
	}

	secondNode := DoubleNode{
		nodeValue: 2,
		left:      &rootNode,
		right:     nil,
	}

	rootNode.right = &secondNode

	assert.Equal(t, rootNode.right.nodeValue, 2)
	assert.Equal(t, secondNode.left.nodeValue, 1)
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
		left:      nil,
		right:     nil,
	}

	assert.Equal(t, tree.root.nodeValue, 3)
}

func TestInsertFirst(t *testing.T) {
	tree := RBTree{}

	tree.insert(1)

	assert.Equal(t, tree.root.nodeValue, 1)
	assert.Equal(t, tree.root.color, BLACK)
}

func TestValueInsertedAsBinaryTree(t *testing.T) {
	tree := RBTree{}

	tree.insert(5)
	tree.insert(1)
	tree.insert(7)

	assert.Equal(t, tree.root.nodeValue, 5)
	assert.Equal(t, tree.root.left.nodeValue, 1)
	assert.Equal(t, tree.root.right.nodeValue, 7)
}

func TestInsertFirstReds(t *testing.T) {
	tree := RBTree{}

	tree.insert(5)
	tree.insert(1)
	tree.insert(7)

	assert.Equal(t, tree.root.left.color, RED)
	assert.Equal(t, tree.root.right.color, RED)
}

func TestLinkParent(t *testing.T) {
	tree := RBTree{}

	tree.insert(5)
	tree.insert(1)
	tree.insert(7)

	assert.Equal(t, tree.root.left.parent.nodeValue, tree.root.nodeValue)
	assert.Equal(t, tree.root.right.parent.nodeValue, tree.root.nodeValue)
}
