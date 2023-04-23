package main

import "fmt"

type Color int

const (
	RED   Color = iota
	BLACK Color = iota
)

type TreeNode struct {
	nodeValue int
	left      *TreeNode
	right     *TreeNode
	parent    *TreeNode
	color     Color
}

func (node *TreeNode) GrandParent() *TreeNode {
	return node.parent.parent
}

func IsLeftChild(node *TreeNode) bool {
	if node.parent == nil {
	}

	if node.parent.left != nil && node.parent.left.nodeValue == node.nodeValue {
		return true
	}

	return false
}

func (node *TreeNode) ParentOpposite() *TreeNode {
	grandParent := node.parent.parent

	if grandParent.left != nil && grandParent.left.nodeValue == node.parent.nodeValue {
		return grandParent.right
	}

	return grandParent.left
}

func NewTreeNode(nodeValue int, color Color) *TreeNode {
	return &TreeNode{
		nodeValue: nodeValue,
		left:      nil,
		right:     nil,
		parent:    nil,
		color:     color,
	}
}

func IsDoubleRed(node *TreeNode) bool {
	if node.color != RED {
		return false
	}

	if node.parent.color != RED {
		return false
	}

	return true
}

type RBTree struct {
	root *TreeNode
}

func (tree *RBTree) Insert(nodeValue int) {
	if tree.root == nil {
		tree.root = NewTreeNode(nodeValue, BLACK)
		return
	}

	newNode := tree.insertBinary(tree.root, nodeValue)
	tree.rebalanceRB(newNode)
}

func (tree *RBTree) insertBinary(node *TreeNode, newValue int) *TreeNode {
	if node.nodeValue > newValue {
		if node.left == nil {
			node.left = NewTreeNode(newValue, RED)
			node.left.parent = node
			return node.left
		}

		return tree.insertBinary(node.left, newValue)
	}

	if node.right == nil {
		node.right = NewTreeNode(newValue, RED)
		node.right.parent = node
		return node.right
	}

	return tree.insertBinary(node.right, newValue)
}

func (tree *RBTree) rebalanceRB(node *TreeNode) {
	if node.GrandParent() == nil {
		return
	}

	for node.parent != nil && node.GrandParent() != nil &&
		node.parent.color == RED && node.color == RED {
		if !IsLeftChild(node.parent) {
			po := node.ParentOpposite()

			if po != nil && po.color == RED {
				po.color = BLACK
				node.parent.color = BLACK
				node.GrandParent().color = RED
				node = node.GrandParent()
			} else {
				if IsLeftChild(node) {
					tree.rotateRightSingle(node)
					node = node.right
				}

				tree.rotateLeft(node)
				node = node.parent
				node.color = BLACK
				if node.parent != nil {
					node.parent.color = RED
				}
			}
		} else {
			po := node.ParentOpposite()

			if po != nil && po.color == RED {
				po.color = BLACK
				node.parent.color = BLACK
				node.GrandParent().color = RED
				node = node.GrandParent()
			} else {
				if !IsLeftChild(node) {
					tree.rotateLeftSingle(node)
					node = node.left
				}

				tree.rotateRight(node)

				node = node.parent
				node.color = BLACK
				if node.parent != nil {
					node.parent.color = RED
				}
			}
		}
	}

	tree.root.color = BLACK
}

func (tree *RBTree) rotateLeft(node *TreeNode) {
	newLeft := node.GrandParent()
	newLeftRight := node.parent.left

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

	newLeft.right = newLeftRight
	if newLeftRight != nil {
		newLeft.right = newLeftRight
	}
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

func (tree *RBTree) rotateRight(node *TreeNode) {
	newRight := node.GrandParent()
	newRightLeft := node.parent.right

	isRightChild := true
	highierParent := newRight.parent

	if highierParent != nil {
		if highierParent.right.nodeValue == newRight.nodeValue {
			isRightChild = true
		} else {
			isRightChild = false
		}
	}

	newParent := node.parent
	newParent.right = newRight
	newParent.parent = newRight.parent

	newRight.left = newRightLeft
	if newRightLeft != nil {
		newRight.left = newRightLeft
	}
	newRight.parent = newParent

	newParent.color = BLACK
	newParent.left.color = RED
	newParent.right.color = RED

	if newParent.parent == nil {
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

func (tree *RBTree) rotateRightSingle(node *TreeNode) {
	grandParent := node.GrandParent()
	newParent := node
	newChild := node.parent
	newRightLeft := node.right

	grandParent.right = nil
	newParent.parent = nil
	newChild.parent = nil
	newChild.left = nil

	grandParent.right = newParent
	newParent.parent = grandParent

	newParent.right = newChild
	newChild.parent = newParent

	newChild.left = newRightLeft
	if newRightLeft != nil {
		newRightLeft.parent = newChild
	}
}

func (tree *RBTree) rotateLeftSingle(node *TreeNode) {
	grandParent := node.GrandParent()
	newParent := node
	newChild := node.parent
	newLeftRight := node.left

	grandParent.left = nil
	newParent.parent = nil
	newChild.parent = nil
	newChild.right = nil

	grandParent.left = newParent
	newParent.parent = grandParent

	newParent.left = newChild
	newChild.parent = newParent

	newChild.right = newLeftRight
	if newLeftRight != nil {
		newLeftRight.parent = newChild
	}
}

func (tree *RBTree) Delete(nodeValue int) {
	targetNode := tree.findNode(nodeValue)

	if targetNode == nil {
		return
	}

	currentNode := targetNode
	originalColor := currentNode.color

	fixNode := currentNode

	if targetNode.left == nil {
		fixNode = targetNode.right
		tree.swapNode(targetNode, targetNode.right)
	} else if targetNode.right == nil {
		fixNode = targetNode.left
		tree.swapNode(targetNode, targetNode.left)
	} else {
		currentNode = tree.findMinimumNode(targetNode.right)
		originalColor = currentNode.color
		fixNode = currentNode.right
		if currentNode.parent == targetNode {
			fixNode.parent = targetNode
		} else {
			tree.swapNode(currentNode, currentNode.right)
			currentNode.right = targetNode.right
			currentNode.right.parent = currentNode
		}

		tree.swapNode(targetNode, currentNode)
		currentNode.left = targetNode.left
		currentNode.left.parent = currentNode
		currentNode.color = targetNode.color
	}

	if originalColor == BLACK {
		return
	}

	//TODO: fixDelete
	tree.fixDelete(fixNode)
}

func (tree *RBTree) findNode(nodeValue int) *TreeNode {
	current := tree.root

	for current != nil {
		if current.nodeValue == nodeValue {
			break
		}

		if current.nodeValue > nodeValue {
			current = current.left
			continue
		}

		current = current.right
		continue
	}

	return current
}

func (tree *RBTree) swapNode(from *TreeNode, to *TreeNode) {
	if from.parent == nil {
		tree.root = to
		return
	}

	if from.parent.left != nil && from.nodeValue == from.parent.left.nodeValue {
		from.parent.left = to
		to.parent = from.parent
		return
	}

	if from.parent.right != nil && from.nodeValue == from.parent.right.nodeValue {
		from.parent.right = to
		to.parent = from.parent
		return
	}
}

func (tree *RBTree) findMinimumNode(targetNode *TreeNode) *TreeNode {
	minimumNode := targetNode

	for targetNode.left != nil {
		minimumNode = targetNode.left
	}

	return minimumNode
}

func (tree *RBTree) fixDelete(fixNode *TreeNode) {
	current := fixNode

	for current != tree.root && current.color == BLACK {
		if IsLeftChild(current) {
			sibling := current.parent.right
			if sibling != nil && sibling.color == RED {
				sibling.color = BLACK
				current.parent.color = RED
				tree.rotateLeftSingle(current.parent)

				sibling = current.parent.right
			}

			if sibling != nil &&
				sibling.left != nil && sibling.left.color == BLACK &&
				sibling.right != nil && sibling.right.color == BLACK {
				sibling.color = RED
				current = current.parent
			} else {
				if sibling.right != nil && sibling.right.color == BLACK {
					if sibling.left != nil {
						sibling.left.color = BLACK
					}

					sibling.color = RED
					tree.rotateRightSingle(sibling)
					sibling = current.parent.right
				}

				sibling.color = current.parent.color
				current.parent.color = BLACK
				sibling.right.color = BLACK
				tree.rotateLeftSingle(current.parent)
				current = tree.root
			}
		} else {
			sibling := current.parent.left
			if sibling != nil && sibling.color == RED {
				sibling.color = BLACK
				current.parent.color = RED
				tree.rotateRightSingle(current.parent)

				sibling = current.parent.left
			}

			if sibling != nil &&
				sibling.left != nil && sibling.left.color == BLACK &&
				sibling.right != nil && sibling.right.color == BLACK {
				sibling.color = RED
				current = current.parent
			} else {
				if sibling.left != nil && sibling.left.color == BLACK {
					if sibling.right != nil {
						sibling.right.color = BLACK
					}

					sibling.color = RED
					tree.rotateLeftSingle(sibling)
					sibling = current.parent.left
				}

				sibling.color = current.parent.color
				current.parent.color = BLACK
				sibling.right.color = BLACK
				tree.rotateRightSingle(current.parent)
				current = tree.root
			}
		}

		current.color = BLACK
	}
}

func main() {
	fmt.Printf("start from here")
}
