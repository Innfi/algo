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

func newDoubleNode(nodeValue int, color Color) *DoubleNode {
	return &DoubleNode{
		nodeValue: nodeValue,
		left:      nil,
		right:     nil,
		parent:    nil,
		color:     color,
	}
}

func (tree *RBTree) insert(nodeValue int) {
	if tree.root == nil {
		tree.root = newDoubleNode(nodeValue, BLACK)
		return
	}

	if tree.root.nodeValue > nodeValue {
		tree.root.left = newDoubleNode(nodeValue, RED)
		tree.root.left.parent = tree.root

		return
	}

	if tree.root.nodeValue < nodeValue {
		tree.root.right = newDoubleNode(nodeValue, RED)
		tree.root.right.parent = tree.root

		return
	}

}

func main() {
	fmt.Printf("start from here")
}
