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

func IsDoubleRed(node *DoubleNode) bool {
	if node.color != RED {
		return false
	}

	if node.parent.color != RED {
		return false
	}

	return true
}

func (tree *RBTree) Insert(nodeValue int) {
	if tree.root == nil {
		tree.root = NewDoubleNode(nodeValue, BLACK)
		return
	}

	newNode := tree.insertBinary(tree.root, nodeValue)
	tree.tryRebalance(newNode)
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

func (tree *RBTree) tryRebalance(node *DoubleNode) {
	if !IsDoubleRed(node) {
		return
	}

	grandParent := node.parent.parent
	if grandParent.left == nil {
		if grandParent.right.right == nil && grandParent.right.left != nil {
			tree.rotateRightSingle(node)
			tree.rotateLeft(node.right)
			return
		}

		tree.rotateLeft(node)
		return
	}

	if grandParent.right == nil {
		tree.rotateRight(node)
		return
	}

	tree.recolor(node)
	return
}

func (tree *RBTree) rotateLeft(node *DoubleNode) {
	newLeft := node.parent.parent

	isRightChild := true
	highierParent := newLeft.parent

	if highierParent != nil {
		if highierParent.right.nodeValue == newLeft.nodeValue {
			isRightChild = true
		} else {
			isRightChild = false
		}
	}

	newParent := node.parent
	newParent.left = newLeft
	newParent.parent = newLeft.parent

	newLeft.right = nil
	newLeft.parent = newParent

	newParent.color = BLACK
	newParent.left.color = RED
	newParent.right.color = RED

	if highierParent == nil {
		tree.root = newParent
		tree.root.color = BLACK
		return
	}

	if isRightChild {
		highierParent.right = newParent
		newParent.parent = highierParent
	} else {
		highierParent.left = newParent
		newParent.parent = highierParent
	}
}

func (tree *RBTree) rotateRight(node *DoubleNode) {
	newRight := node.parent.parent

	newParent := node.parent
	newParent.right = newRight
	newParent.parent = newRight.parent

	newRight.left = nil
	newRight.parent = newParent
	newRight.color = RED

	if newParent.parent == nil {
		tree.root = newParent
		tree.root.color = BLACK
	}
}

func (tree *RBTree) rotateRightSingle(node *DoubleNode) {
	grandParent := node.parent.parent
	newParent := node
	newChild := node.parent

	grandParent.right = nil
	newParent.parent = nil
	newChild.parent = nil
	newChild.left = nil

	grandParent.right = newParent
	newParent.parent = grandParent

	newParent.right = newChild
	newChild.parent = newParent
}

func (tree *RBTree) recolor(node *DoubleNode) {
	grandParent := node.parent.parent
	grandParent.left.color = BLACK
	grandParent.right.color = BLACK
}

func main() {
	fmt.Printf("start from here")
}
