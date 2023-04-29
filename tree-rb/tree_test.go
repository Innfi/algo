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
  - intermediate case 1: 10 5 20 30 25
  - medium case 1: 1 2 3 4 5 6 7 8
	- medium case 2: 8 7 6 5 4 3 2 1
- utility function for instance creation
- constraint checker
*/

func TestLink(t *testing.T) {
	rootNode := TreeNode{
		nodeValue: 1,
		left:      nil,
		right:     nil,
	}

	secondNode := TreeNode{
		nodeValue: 2,
		left:      &rootNode,
		right:     nil,
	}

	rootNode.right = &secondNode

	assert.Equal(t, rootNode.right.nodeValue, 2)
	assert.Equal(t, secondNode.left.nodeValue, 1)
}

func TestStateChange(t *testing.T) {
	tree := RBTree{}
	tree.root = &TreeNode{
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

	assert.Equal(t, CheckConstraintColor(root), true)

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
	root := NewTreeNode(10, BLACK)
	root.left = NewTreeNode(5, RED)
	root.left.parent = root

	root.left.left = NewTreeNode(3, BLACK)
	root.left.left.parent = root.left

	root.left.right = NewTreeNode(7, RED)
	root.left.right.parent = root.left

	root.left.left.left = NewTreeNode(1, BLACK)
	root.left.left.left.parent = root.left.left

	assert.Equal(t, CheckConstraintDepth(root), false)
}

func TestSimpleRestructureRight(t *testing.T) {
	tree := CreatePreset([]int{10, 5, 20, 30, 25})

	rightNode := tree.root.right
	assert.Equal(t, rightNode.nodeValue, 25)

	assert.Equal(t, CheckConstraintColor(tree.root), true)
	assert.Equal(t, CheckConstraintDepth(tree.root), true)
}

func TestSimpleRestructureLeft(t *testing.T) {
	tree := CreatePreset([]int{10, 5, 20, 1, 3})

	leftNode := tree.root.left
	assert.Equal(t, leftNode.nodeValue, 3)

	assert.Equal(t, CheckConstraintColor(tree.root), true)
	assert.Equal(t, CheckConstraintDepth(tree.root), true)
}

func TestMediumCase1(t *testing.T) {
	tree := CreatePreset([]int{1, 2, 3, 4, 5, 6, 7, 8})

	assert.Equal(t, CheckConstraintColor(tree.root), true)
	assert.Equal(t, CheckConstraintDepth(tree.root), true)
}

func TestMediumCase2(t *testing.T) {
	tree := CreatePreset([]int{8, 7, 6, 5, 4, 3, 2, 1})

	assert.Equal(t, CheckConstraintColor(tree.root), true)
	assert.Equal(t, CheckConstraintDepth(tree.root), true)
}

func TestDeleteNodeSingle(t *testing.T) {
	tree := CreatePreset([]int{1})

	tree.Delete(1)

	assert.Equal(t, tree.root == nil, true)
}

func TestDeleteChildrenSimple(t *testing.T) {
	tree := CreatePreset([]int{2, 1, 3})

	tree.Delete(1)

	assert.Equal(t, tree.root.nodeValue, 2)
}

func TestDeleteRootSimple(t *testing.T) {
	tree := CreatePreset([]int{2, 1, 3})

	tree.Delete(2)

	assert.Equal(t, tree.root.nodeValue, 3)
}

func TestDeleteMiddle(t *testing.T) {
	tree := CreatePreset([]int{2, 1, 3, 4})

	tree.Delete(3)

	assert.Equal(t, tree.root.nodeValue, 2)
	assert.Equal(t, tree.root.right.nodeValue, 4)
	assert.Equal(t, tree.root.right.color, BLACK)
}
