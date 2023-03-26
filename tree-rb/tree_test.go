package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

/*
- mark node as red/black according to constraints
  - every node is colored either black or red
	- root node is black
	- every leaf node is black
	- children of red node is black
	- for each node, any simple path from the node to its leaves
	  has the same number of black nodes (same depth)

TOOD
--------------------------------------------------------------------------------
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

	tree.Insert(1)

	assert.Equal(t, tree.root.nodeValue, 1)
	assert.Equal(t, tree.root.color, BLACK)
}

func TestValueInsertedAsBinaryTree(t *testing.T) {
	tree := RBTree{}

	tree.Insert(5)
	tree.Insert(1)
	tree.Insert(7)

	assert.Equal(t, tree.root.nodeValue, 5)
	assert.Equal(t, tree.root.left.nodeValue, 1)
	assert.Equal(t, tree.root.right.nodeValue, 7)
}

func TestInsertFirstReds(t *testing.T) {
	tree := RBTree{}

	tree.Insert(5)
	tree.Insert(1)
	tree.Insert(7)

	assert.Equal(t, tree.root.left.color, RED)
	assert.Equal(t, tree.root.right.color, RED)
}

func TestLinkParent(t *testing.T) {
	tree := RBTree{}

	tree.Insert(5)
	tree.Insert(1)
	tree.Insert(7)

	assert.Equal(t, tree.root.left.parent.nodeValue, tree.root.nodeValue)
	assert.Equal(t, tree.root.right.parent.nodeValue, tree.root.nodeValue)
}

func TestInsertNodeAsBinaryTree(t *testing.T) {
	tree := RBTree{}

	tree.Insert(5)
	tree.Insert(1)
	tree.Insert(10)
	tree.Insert(7)
	tree.Insert(3)

	firstLeaf := tree.root.left.right
	secondLeaf := tree.root.right.left

	assert.Equal(t, firstLeaf.nodeValue, 3)
	assert.Equal(t, secondLeaf.nodeValue, 7)
}

func TestSimpleRecolor(t *testing.T) {
	tree := RBTree{}

	tree.Insert(5)
	tree.Insert(1)
	tree.Insert(10)
	tree.Insert(7)

	root := tree.root
	firstLeft := root.left
	firstRight := root.right

	assert.Equal(t, root.color, BLACK)
	assert.Equal(t, firstLeft.color, BLACK)
	assert.Equal(t, firstRight.color, BLACK)

	assert.Equal(t, firstRight.left.nodeValue, 7)
	assert.Equal(t, firstRight.left.color, RED)
}
