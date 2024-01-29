package main

import "fmt"

type Node struct {
	parent *Node
	next   *Node
	kvset  map[int]ValueDef
}

type ValueDef struct {
	child *Node
	data  string
}

type BPlusTree struct {
	root *Node
}

func InitBPlusTree() *BPlusTree {
	return &BPlusTree{
		root: nil,
	}
}

func (tree *BPlusTree) Insert(key int, data string) bool {
	if tree.root == nil {
		tree.root = &Node{
			parent: nil,
			next:   nil,
			kvset:  make(map[int]ValueDef),
		}
	}

	// not implemented
	return false
}

func main() {
	fmt.Printf("start from here")
}
