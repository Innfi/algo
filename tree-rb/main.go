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
	if node == nil || node.parent == nil {
		return true
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
					tree.rotateRight(node)
					node = node.right
				}

				tree.rotateLeft(node.parent)

				node = node.parent
				node.color = BLACK
				if node.parent != nil {
					node.parent.color = RED
				}
				if node.left != nil {
					node.left.color = RED
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
					tree.rotateLeft(node)
					node = node.left
				}

				tree.rotateRight(node.parent)

				node = node.parent
				node.color = BLACK
				if node.parent != nil {
					node.parent.color = RED
				}
				if node.right != nil {
					node.right.color = RED
				}
			}
		}
	}

	tree.root.color = BLACK
	tree.root.parent = nil
}

func (tree *RBTree) rotateLeft(node *TreeNode) {
	if node.parent == nil {
		return
	}

	grandParent := node.GrandParent()
	if grandParent == nil {
		tree.root = node
		tree.rotateLeftAndReassign(node)

		return
	}

	isLeftChild := IsLeftChild(grandParent.right)

	tree.rotateLeftAndReassign(node)

	if isLeftChild {
		grandParent.left = node
	} else {
		grandParent.right = node
	}

	node.parent = grandParent
}

func (tree *RBTree) rotateLeftAndReassign(node *TreeNode) {
	newLeft := node.parent
	newLeftRight := node.left

	node.left = newLeft
	newLeft.parent = node

	newLeft.right = newLeftRight
	if newLeftRight != nil {
		newLeftRight.parent = newLeft
	}
}

func (tree *RBTree) rotateRight(node *TreeNode) {
	if node.parent == nil {
		return
	}

	grandParent := node.GrandParent()

	if grandParent == nil {
		tree.root = node
		tree.rotateRightReassign(node)

		return
	}

	isLeftChild := true
	if grandParent.right != nil && grandParent.right.nodeValue == node.parent.nodeValue {
		isLeftChild = false
	}
	// isLeftChild := IsLeftChild(grandParent.right)

	tree.rotateRightReassign(node)

	if isLeftChild {
		grandParent.left = node
	} else {
		grandParent.right = node
	}

	node.parent = grandParent
}

func (tree *RBTree) rotateRightReassign(node *TreeNode) {
	newRight := node.parent
	newRightLeft := node.right

	node.right = newRight
	newRight.parent = node

	newRight.left = newRightLeft
	if newRightLeft != nil {
		newRightLeft.parent = newRight
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
		if currentNode.parent.nodeValue == targetNode.nodeValue && fixNode != nil {
			fixNode.parent = targetNode
		} else {
			tree.swapNode(currentNode, currentNode.right)
			currentNode.right = targetNode.right

			if currentNode.right != nil {
				currentNode.right.parent = currentNode
			}
		}

		tree.swapNode(targetNode, currentNode)
		currentNode.left = targetNode.left

		if currentNode.left != nil {
			currentNode.left.parent = currentNode
		}

		currentNode.color = targetNode.color
	}

	if originalColor == BLACK || fixNode == nil {
		currentNode.color = targetNode.color
		tree.root.color = BLACK
		return
	}

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
		if to != nil {
			to.parent = from.parent
		}
		return
	}

	if from.parent.right != nil && from.nodeValue == from.parent.right.nodeValue {
		from.parent.right = to
		if to != nil {
			to.parent = from.parent
		}
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
				tree.rotateLeft(current.parent)

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
					tree.rotateRight(sibling)
					sibling = current.parent.right
				}

				sibling.color = current.parent.color
				current.parent.color = BLACK
				sibling.right.color = BLACK
				tree.rotateLeft(current.parent)
				current = tree.root
			}
		} else {
			sibling := current.parent.left
			if sibling != nil && sibling.color == RED {
				sibling.color = BLACK
				current.parent.color = RED
				tree.rotateRight(current.parent)

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
					tree.rotateLeft(sibling)
					sibling = current.parent.left
				}

				sibling.color = current.parent.color
				current.parent.color = BLACK
				sibling.right.color = BLACK
				tree.rotateRight(current.parent)
				current = tree.root
			}
		}

		current.color = BLACK
	}
}

func main() {
	fmt.Printf("start from here")
}
