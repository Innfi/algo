package main

import "fmt"

type KVSet struct {
	key   int
	value string
}

type TreeNode struct {
	kvSet []KVSet

	children *TreeNode
	next     *TreeNode
}

func NewTreeNode() *TreeNode {
	return &TreeNode{
		kvSet: []KVSet{},
	}
}

func (node *TreeNode) Insert(kvSet KVSet) {
	node.kvSet = append(node.kvSet, kvSet)
}

func main() {
	fmt.Printf("start from here")
}
