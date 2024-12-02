# Day 2: Advent of Code Solution

[Link to the problem](https://adventofcode.com/2024/day/2)

## Problem Description

### Part 1: Safe "Red-Nosed" Reports

Given a set of reports in the form of lists of numbers:

- A report is considered **safe** if:
  1. All elements follow a constant order (either strictly increasing or strictly decreasing).
  2. The difference between consecutive elements is between 1 and 3, inclusive.

The goal is to count the total number of **safe** reports.

---

### Part 2: Safe "Red-Nosed" Reports with Dampener

A report can be made **safe** by removing one element.  
The goal is to count the number of reports that can be made safe by removing a single element.

## Solution

### Part 1: Safe Reports Calculation

The function `red_nosed_reports` iterates through each report and checks if it follows the safe conditions:

- It checks if the report is either strictly increasing or strictly decreasing.
- It checks if the difference between each consecutive pair of elements is between 1 and 3.

**Time Complexity**:

- Iterating through each report: \( O(n) \)
- Checking conditions: \( O(m) \), where \( m \) is the length of each report  
  **Overall**: \( O(n \times m) \)

**Space Complexity**:

- \( O(1) \) additional space, as we're not storing additional data structures.

---

### Part 2: Safe Reports with Dampener Calculation

The function `red_nosed_reports_with_dampener` checks if a report can be made safe by removing one element. It tries to remove each element in turn and checks if the resulting report is safe.

**Time Complexity**:

- Iterating through each report: \( O(n) \)
- Checking if a report is safe after removing an element: \( O(m) \)  
  **Overall**: \( O(n \times m^2) \)

**Space Complexity**:

- \( O(m) \) for cloning and modifying the report.
