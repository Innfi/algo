package main

import "fmt"

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

func CheckConstraintDepth(node *DoubleNode) bool {
	depths := []int{}

	Depth(node, &depths)

	if len(depths) <= 0 {
		return false
	}

	current := depths[0]
	for _, elem := range depths {
		if current != elem {
			return false
		}
	}

	return true
}

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

func PrintTreeBFS(node *DoubleNode) {
	if node == nil {
		return
	}

	traverseTarget := []*DoubleNode{}
	nodesByBFS := []*DoubleNode{}

	traverseTarget = append(traverseTarget, node)

	for len(traverseTarget) > 0 {
		current := traverseTarget[0]
		traverseTarget = traverseTarget[1:]

		nodesByBFS = append(nodesByBFS, current)

		if current.left != nil {
			traverseTarget = append(traverseTarget, current.left)
		}
		if current.right != nil {
			traverseTarget = append(traverseTarget, current.right)
		}
	}

	for _, elem := range nodesByBFS {
		fmt.Printf("value: %d, color: %d \n", elem.nodeValue, elem.color)
	}
}
