package main

import "fmt"

type Color int

const (
	RED   Color = iota
	BLACK Color = iota
)

type Node struct {
	nodeValue int
	next      *Node
}

type DoubleNode struct {
	nodeValue int
	left      *DoubleNode
	right     *DoubleNode
	parent    *DoubleNode
	color     Color
}

type RBTree struct {
	root *DoubleNode
}

func NewDoubleNode(nodeValue int, color Color) *DoubleNode {
	return &DoubleNode{
		nodeValue: nodeValue,
		left:      nil,
		right:     nil,
		parent:    nil,
		color:     color,
	}
}

func (tree *RBTree) Insert(nodeValue int) {
	if tree.root == nil {
		tree.root = NewDoubleNode(nodeValue, BLACK)
		return
	}

	newNode := tree.insertBinary(tree.root, nodeValue)
	tree.tryRecolor(newNode)
}

func (tree *RBTree) insertBinary(node *DoubleNode, newValue int) *DoubleNode {
	if node.nodeValue > newValue {
		if node.left == nil {
			node.left = NewDoubleNode(newValue, RED)
			node.left.parent = node
			return node.left
		}

		return tree.insertBinary(node.left, newValue)
	}

	if node.right == nil {
		node.right = NewDoubleNode(newValue, RED)
		node.right.parent = node
		return node.right
	}

	return tree.insertBinary(node.right, newValue)
}

// to be renamed as tryRebalance
func (tree *RBTree) tryRecolor(node *DoubleNode) {
	fmt.Printf("node: %d\n", node.nodeValue)
	if node.parent == nil {
		return
	}

	if node.color != RED || node.parent.color != RED {
		return
	}

	grandParent := node.parent.parent
	grandParent.left.color = BLACK
	grandParent.right.color = BLACK
}

func main() {
	fmt.Printf("start from here")
}
