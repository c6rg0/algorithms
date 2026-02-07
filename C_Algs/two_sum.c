#include <stdio.h>

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* twoSum(int* nums, int numsSize, int target, int* returnSize) {
	int *returnArray = malloc(2 * sizeof(int));
	int i;
	for (i = 0; i < numsSize - 1; i++)
	{
		if ((i + 1) < (numsSize - 1))
		{
			if (nums[i] + nums[i + 1] == target)
			{
				returnArray[0] = i;
				returnArray[1] = i + 1;
				return returnArray;
			}

			if (nums[i + 2] == NULL)
			{
				if (nums[i] + nums[i + 2] == target)
				{
					returnArray[0] = i;
					returnArray[1] = i + 2;
					return returnArray;
				}
			}
		}
	}
}

int main(){
	int nums[] = {2, 7, 11, 15};
	int* numsSize = sizeof(nums) / sizeof(nums[0]);
	int target = 9;
	int* returnSize = sizeof(nums[0]) + sizeof(nums[1]);
	twoSum(nums, numsSize, target, returnSize);
}

