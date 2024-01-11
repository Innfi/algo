// https://leetcode.com/problems/merge-sorted-array/?envType=study-plan-v2&envId=top-interview-150

import assert from 'assert';

function merge(nums1: number[], m: number, nums2: number[], n: number): void {
  let leftIndex = 0;
  let rightIndex = 0;

  while(true) {
    if (leftIndex <= m-1 && nums1[leftIndex] < nums2[rightIndex]) {
      leftIndex += 1;
      continue;
    }

    console.log(`leftIndex: ${leftIndex}`);
    nums1.splice(leftIndex, 0, nums2[rightIndex]);
    leftIndex += 1;
    rightIndex += 1;
    if (rightIndex === n) break;

  }
};

describe('testing', () => {
  it('array splice', () => {
    const input = [1, 2, 3, 5];

    input.splice(3, 0, 4);
    assert.deepStrictEqual(input, [1, 2, 3, 4, 5]);

    input.splice(3, 0, 6);
    assert.deepStrictEqual(input, [1, 2, 3, 6, 4, 5]);
  });

  it('array splice2', () => {
    const input = [1];

    input.splice(1, 0, 2);
    assert.deepStrictEqual(input, [1, 2]);

    input.splice(2, 0, 3);
    assert.deepStrictEqual(input, [1, 2, 3]);
  });
  
  it('basic case 1', () => {
    const left = [1, 2, 3];
    const right = [4, 5, 6];

    merge(left, left.length, right, right.length);

    assert.deepStrictEqual(left, [1, 2, 3, 4, 5, 6]);
  });
});