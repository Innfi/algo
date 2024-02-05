package main

import "fmt"

const KVLIST_DEGREE = 5

type Node struct {
	parent *Node
	next   *Node
	kvList []KVList
}

func CreateNode() *Node {
	return &Node{
		parent: nil,
		next:   nil,
		kvList: make([]KVList, 0, KVLIST_DEGREE),
	}
}

type KVList struct {
	key   int
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
			kvList: make([]KVList, 0, KVLIST_DEGREE),
		}
	}

	// not implemented
	return false
}

func SplitKVList(node *Node) {

}

func main() {
	fmt.Printf("start from here")
}
