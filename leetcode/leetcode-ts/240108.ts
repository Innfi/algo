/**
 * https://leetcode.com/problems/range-sum-of-bst
 */

import assert from 'assert';

class TreeNode {
    val: number
    left: TreeNode | null
    right: TreeNode | null
    constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
        this.val = (val===undefined ? 0 : val)
        this.left = (left===undefined ? null : left)
        this.right = (right===undefined ? null : right)
    }
}

function rangeSumBST(root: TreeNode | null, low: number, high: number): number {
  if (!root) {
    console.log(`empty; return`);
    return 0;
  }
  if (root.val < low || root.val > high) {
    console.log(`out of range: ${root.val}; return`);
    return rangeSumBST(root.left, low, high) + rangeSumBST(root.right, low, high);
  }

  console.log(`value: ${root.val}`);

  return root.val + rangeSumBST(root.left, low, high) + rangeSumBST(root.right, low, high);
}

describe('range sum', () => {
  it('basic test2', () => {
    const root = new TreeNode(10);

    root.left = new TreeNode(5);
    root.left.left = new TreeNode(3);
    root.left.left.left = new TreeNode(1);

    root.left.right = new TreeNode(7);
    root.left.right.left = new TreeNode(6);

    root.right = new TreeNode(15);
    root.right.left = new TreeNode(13);
    root.right.right = new TreeNode(18);

    assert.strictEqual(rangeSumBST(root, 6, 10), 23);
  });
});