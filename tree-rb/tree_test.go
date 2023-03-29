package main

import (
	"fmt"
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
- constraint checker
- insert nodes
  - intermediate case 1: 1 2 3 4 5
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
*/

func CreatePreset(input []int) RBTree {
	tree := RBTree{}

	for _, elem := range input {
		tree.Insert(elem)
	}

	return tree
}

func CheckConstraintColor(node *DoubleNode) bool {
	if node == nil {
		return true
	}

	if node.parent == nil && node.color != BLACK {
		return false
	}

	if node.color == RED && node.parent.color == RED {
		return false
	}

	if CheckConstraintColor(node.left) == false {
		return false
	}

	if CheckConstraintColor(node.right) == false {
		return false
	}

	return true
}

// func CheckConstraintDepth(node *DoubleNode) bool {
// 	return false
// }
//

func Depth(node *DoubleNode, depths *[]int) {
	if node == nil {
		return
	}

	if node.left != nil {
		Depth(node.left, depths)
	}

	if node.right != nil {
		Depth(node.right, depths)
	}

	if node.parent == nil {
		return
	}

	current := node
	depth := 0
	for current != nil {
		if current.color == BLACK {
			depth += 1
		}

		current = current.parent
	}

	*depths = append(*depths, depth)
}

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

	for _, elem := range depths {
		fmt.Printf("depth: %d\n", elem)
	}
}
