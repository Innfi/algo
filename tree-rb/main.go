package main

import "fmt"

type Node struct {
	nodeValue int
	next      *Node
}

type DoubleNode struct {
	nodeValue int
	prev      *DoubleNode
	next      *DoubleNode
}

type RBTree struct {
	root *DoubleNode
}

func (tree *RBTree) insert(nodeValue int) {
	fmt.Printf("RBTree.insert] \n")

	if tree.root == nil {
		fmt.Printf("RBTree.insert] empty root \n")

		tree.root = &DoubleNode{
			nodeValue: nodeValue,
			prev:      nil,
			next:      nil,
		}
	}
	fmt.Printf("RBTree.insert] root.nodeValue: %d\n", tree.root.nodeValue)
}

func main() {
	fmt.Printf("start from here")
}
