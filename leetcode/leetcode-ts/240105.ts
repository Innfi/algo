https://leetcode.com/problems/remove-duplicates-from-sorted-array/submissions/?envType=study-plan-v2&envId=top-interview-150

function removeDuplicates(nums: number[]): number {
    for(let i=1;i<nums.length;i++) {
        if (nums[i-1] === nums[i]) {
            nums.splice(i, 1);
            i -= 1;
        }
    }

    return nums.length;
};