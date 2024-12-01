# Day 1: Advent of Code Solution

[Link to the problem](https://adventofcode.com/2024/day/1)

This solution calculates two results for given input arrays:

1. The sum of the absolute differences between elements of two arrays after sorting.
2. A weighted sum based on the frequency of elements in the second array.

## Problem Description

Given two arrays, perform the following tasks:

1. **Part 1**: Calculate the sum of absolute differences between corresponding elements of the arrays after sorting.
2. **Part 2**: Compute a weighted sum where each element of the first array is multiplied by its frequency in the second array.

## Solution

### Part 1: Sum of Absolute Differences

The `day1_1` function:

- Sorts both arrays.
- Iterates through both arrays and computes the sum of absolute differences for corresponding elements.

**Time Complexity**:

- Sorting: \( O(n \log n) \)
- Summing: \( O(n) \)  
  **Overall**: \( O(n \log n) \)

**Space Complexity**:

- Sorting uses \( O(n) \) additional space.  
  **Overall**: \( O(n) \)

---

### Part 2: Weighted Sum

The `day1_2` function:

- Builds a frequency map (`HashMap`) for elements in the second array.
- Iterates through the first array, calculating the weighted sum using the frequency map.

**Time Complexity**:

- Building the frequency map: \( O(n) \)
- Calculating the weighted sum: \( O(n) \)  
  **Overall**: \( O(n) \)

**Space Complexity**:

- Storing the frequency map: \( O(n) \)  
  **Overall**: \( O(n) \)
