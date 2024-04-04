package main

import (
	"fmt"
)

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

	return tree.insertRecursive(tree.root, key, data)
}

func (tree *BPlusTree) insertRecursive(node *Node, key int, data string) bool {
	node.kvList = append(node.kvList, KVList{
		key:  key,
		data: data,
	})

	if len(node.kvList) < KVLIST_DEGREE {
		return true
	}

	SplitKVList(node)

	//TODO: check threshold event to parent

	return true
}

func SplitKVList(node *Node) {
	listLen := len(node.kvList)
	middle := uint32(listLen / 2)

	leftNode := CreateNode()
	for _, elem := range node.kvList[0:middle] {
		leftNode.kvList = append(leftNode.kvList, KVList{
			key:  elem.key,
			data: elem.data,
		})
	}
	rightNode := CreateNode()
	for _, elem := range node.kvList[middle:listLen] {
		rightNode.kvList = append(rightNode.kvList, KVList{
			key:  elem.key,
			data: elem.data,
		})
	}
	leftNode.next = rightNode

	node.kvList = make([]KVList, 0, KVLIST_DEGREE)
	node.kvList = append(node.kvList, KVList{
		key:   leftNode.kvList[0].key,
		child: leftNode,
	})
	node.kvList = append(node.kvList, KVList{
		key:   rightNode.kvList[0].key,
		child: rightNode,
	})
}

func main() {
	fmt.Printf("start from here")
}
