// https://leetcode.com/problems/binary-tree-preorder-traversal/description/

package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func preorderTraversal(root *TreeNode) []int {
	output := []int{}

	preorderRecursive(root, output)

	return output
}

func preorderRecursive(node *TreeNode, output []int) {
	if node == nil {
		return
	}

	output = append(output, node.Val)

	preorderRecursive(node.Left, output)
	preorderRecursive(node.Right, output)
}

func main() {

}
