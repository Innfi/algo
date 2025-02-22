// https://leetcode.com/problems/merge-sorted-array/?envType=study-plan-v2&envId=top-interview-150

/**
 * 
 * function merge(nums1: number[], m: number, nums2: number[], n: number): void {
    let i = m - 1;
    let j = n - 1;
    let k = m + n - 1;
    
    while (j >= 0) {
        if (i >= 0 && nums1[i] > nums2[j]) {
            nums1[k--] = nums1[i--];
        } else {
            nums1[k--] = nums2[j--];
        }
    }
};
 * 
 */

import assert from 'assert';

function merge(nums1: number[], m: number, nums2: number[], n: number): void {
  let leftIndex = 0;
  let rightIndex = 0;

  while(leftIndex < nums1.length) {
    if (nums1[leftIndex] > 0) {
      leftIndex += 1;
      continue;
    }

    nums1.splice(leftIndex, nums1.length-leftIndex);
  }
  leftIndex = 0;

  while(rightIndex < nums2.length) {
    if (leftIndex < nums1.length && nums1[leftIndex] < nums2[rightIndex]) {
      leftIndex += 1;

      continue;
    } 
    
    nums1.splice(leftIndex, 0, nums2[rightIndex]);
    rightIndex += 1;
    continue;
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
    const left = [1, 2, 3, 0, 0, 0];
    const right = [4, 5, 6];

    merge(left, left.length, right, right.length);

    assert.deepStrictEqual(left, [1, 2, 3, 4, 5, 6]);
  });

  it('basic case 2', () => {
    const left = [4, 5, 6];
    const right = [1, 2, 3];

    merge(left, left.length, right, right.length);

    assert.deepStrictEqual(left, [1, 2, 3, 4, 5, 6]);
  });

  it('surrounding case', () => {
    const left = [7, 8, 9];
    const right = [1, 2, 11];

    merge(left, left.length, right, right.length);

    assert.deepStrictEqual(left, [1, 2, 7, 8, 9, 11]);
  });

  it('overlapping case', () => {
    const left = [1, 2, 10];
    const right = [5, 6, 7];

    merge(left, left.length, right, right.length);

    assert.deepStrictEqual(left, [1, 2, 5, 6, 7, 10]);
  });

  it('same number: single case', () => {
    const left = [1, 4, 7];
    const right = [2, 3, 4];

    merge(left, left.length, right, right.length);

    assert.deepStrictEqual(left, [1, 2, 3, 4, 4, 7]);
  });

  it('same number: multiple case', () => {
    const left = [1, 2, 3];
    const right = [1, 2, 3];

    merge(left, left.length, right, right.length);

    assert.deepStrictEqual(left, [1, 1, 2, 2, 3, 3]);
  });
});
