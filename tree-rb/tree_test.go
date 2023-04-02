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
  - intermediate case 1: 10 5 20 30 25
  - intermediate case 2: 1 2 3 4 5
- remove node(s)

DONE
--------------------------------------------------------------------------------
- create node and link
- create tree
- insert nodes
  - insert initial node
  - simple recoloring: 2 1 3 4
  - simple left rotation: 1 2 3
  - simple right rotaion: 3 2 1
- utility function for instance creation
- constraint checker
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
	tree := CreatePreset([]int{1})

	tree.Insert(1)

	assert.Equal(t, tree.root.nodeValue, 1)
	assert.Equal(t, CheckConstraintColor(tree.root), true)
}

func TestValueInsertedAsBinaryTree(t *testing.T) {
	tree := CreatePreset([]int{5, 1, 7})

	tree.Insert(5)
	tree.Insert(1)
	tree.Insert(7)

	assert.Equal(t, tree.root.nodeValue, 5)
	assert.Equal(t, tree.root.left.nodeValue, 1)
	assert.Equal(t, tree.root.right.nodeValue, 7)
}

func TestInsertFirstReds(t *testing.T) {
	tree := CreatePreset([]int{5, 1, 7})

	assert.Equal(t, CheckConstraintColor(tree.root), true)
}

func TestLinkParent(t *testing.T) {
	tree := CreatePreset([]int{5, 1, 7})

	assert.Equal(t, tree.root.left.parent.nodeValue, tree.root.nodeValue)
	assert.Equal(t, tree.root.right.parent.nodeValue, tree.root.nodeValue)
}

func TestInsertNodeAsBinaryTree(t *testing.T) {
	tree := CreatePreset([]int{5, 1, 10, 7, 3})

	firstLeaf := tree.root.left.right
	secondLeaf := tree.root.right.left

	assert.Equal(t, firstLeaf.nodeValue, 3)
	assert.Equal(t, secondLeaf.nodeValue, 7)
}

func TestSimpleRecolor(t *testing.T) {
	tree := CreatePreset([]int{5, 1, 10, 7})

	root := tree.root
	firstLeft := root.left
	firstRight := root.right

	assert.Equal(t, CheckConstraintColor(tree.root), true)

	assert.Equal(t, root.color, BLACK)
	assert.Equal(t, firstLeft.color, BLACK)
	assert.Equal(t, firstRight.color, BLACK)

	assert.Equal(t, firstRight.left.nodeValue, 7)
	assert.Equal(t, firstRight.left.color, RED)
}

func TestSimpleLeftRotation(t *testing.T) {
	tree := CreatePreset([]int{1, 2, 3})

	root := tree.root
	left := tree.root.left
	right := tree.root.right

	assert.Equal(t, root.nodeValue, 2)
	assert.Equal(t, left.nodeValue, 1)
	assert.Equal(t, right.nodeValue, 3)

	assert.Equal(t, left.right == nil, true)
}

func TestSimpleRightRotaion(t *testing.T) {
	tree := CreatePreset([]int{3, 2, 1})

	root := tree.root
	left := tree.root.left
	right := tree.root.right

	assert.Equal(t, root.nodeValue, 2)
	assert.Equal(t, left.nodeValue, 1)
	assert.Equal(t, right.nodeValue, 3)

	assert.Equal(t, right.left == nil, true)
}

func TestColorAfterRotation(t *testing.T) {
	tree := CreatePreset([]int{1, 2, 3})

	root := tree.root
	left := tree.root.left
	right := tree.root.right

	assert.Equal(t, root.color, BLACK)
	assert.Equal(t, left.color, RED)
	assert.Equal(t, right.color, RED)

	assert.Equal(t, CheckConstraintColor(tree.root), true)
}

func TestDepthChecker(t *testing.T) {
	tree := CreatePreset([]int{2, 1, 3})

	depths := []int{}
	Depth(tree.root, &depths)

	assert.Equal(t, tree.root.right.nodeValue, 3)
}

func TestDepthCheckerValidCase(t *testing.T) {
	tree := CreatePreset(([]int{10, 5, 15, 1, 7, 13, 17}))

	assert.Equal(t, CheckConstraintDepth(tree.root), true)
}

func TestDepthCheckerInvalidCase(t *testing.T) {
	root := NewDoubleNode(10, BLACK)
	root.left = NewDoubleNode(5, RED)
	root.left.parent = root

	root.left.left = NewDoubleNode(3, BLACK)
	root.left.left.parent = root.left

	root.left.right = NewDoubleNode(7, BLACK)
	root.left.right.parent = root.left

	root.left.left.left = NewDoubleNode(1, RED)
	root.left.left.left.parent = root.left.left

	assert.Equal(t, CheckConstraintDepth(root), false)
}

func TestSimpleRestructure(t *testing.T) {
	// tree := CreatePreset([]int{10, 5, 20, 30, 25})
	tree := CreatePreset([]int{10, 5, 20, 1, 25})

	PrintTreeBFS(tree.root)
}
