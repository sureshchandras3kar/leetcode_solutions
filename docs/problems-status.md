# Problems Status

**20 / 119 complete** — updated 2026-05-08

Legend: ✅ Done · ⬜ Pending

---

## Arrays & Hashing — 15 / 15 ✅

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 1 | Two Sum | Easy | ✅ | Hash map, Two pointer, Brute force |
| 9 | Palindrome Number | Easy | ✅ | Reverse half, String convert, Math |
| 169 | Majority Element | Easy | ✅ | Boyer-Moore, Hash map, Sorting |
| 217 | Contains Duplicate | Easy | ✅ | Hash set, Sorting, Brute force |
| 242 | Valid Anagram | Easy | ✅ | Char count, Sort, Hash map |
| 268 | Missing Number | Easy | ✅ | XOR, Math sum, Hash set |
| 349 | Intersection of Two Arrays | Easy | ✅ | Hash set, Two pointer, Binary search |
| 49 | Group Anagrams | Medium | ✅ | Sort key, Char count map |
| 347 | Top K Frequent Elements | Medium | ✅ | Max heap, Bucket sort |
| 238 | Product of Array Except Self | Medium | ✅ | Prefix/suffix arrays, Two-pass O(1) |
| 36 | Valid Sudoku | Medium | ✅ | Hash sets, Bitmask |
| 128 | Longest Consecutive Sequence | Medium | ✅ | Hash set O(n), Sort |
| 560 | Subarray Sum Equals K | Medium | ✅ | Prefix sum + hash map, Brute force |
| 525 | Contiguous Array | Medium | ✅ | Prefix sum (0→-1 transform) |
| 41 | First Missing Positive | Hard | ✅ | Index as hash, Cycle sort |

---

## Two Pointers — 5 / 5 ✅

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 125 | Valid Palindrome | Easy | ✅ | Two pointers, Clean then check |
| 167 | Two Sum II | Medium | ✅ | Two pointers |
| 15 | 3Sum | Medium | ✅ | Sort + two pointers, Hash set |
| 11 | Container With Most Water | Medium | ✅ | Two pointers greedy, Brute force |
| 42 | Trapping Rain Water | Hard | ✅ | Two pointers, Prefix max, Stack |

---

## Sliding Window — 0 / 6 ⬜

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 121 | Best Time to Buy and Sell Stock | Easy | ⬜ | One-pass min tracking |
| 3 | Longest Substring Without Repeating Characters | Medium | ⬜ | Sliding window + set |
| 424 | Longest Repeating Character Replacement | Medium | ⬜ | Sliding window + freq map |
| 567 | Permutation in String | Medium | ⬜ | Fixed-size window, char count |
| 76 | Minimum Window Substring | Hard | ⬜ | Variable window + need/have counters |
| 239 | Sliding Window Maximum | Hard | ⬜ | Monotonic deque |

---

## Stack — 0 / 7 ⬜

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 20 | Valid Parentheses | Easy | ⬜ | Stack, Hash map for pairs |
| 155 | Min Stack | Medium | ⬜ | Stack of (val, min) pairs |
| 150 | Evaluate Reverse Polish Notation | Medium | ⬜ | Stack |
| 22 | Generate Parentheses | Medium | ⬜ | Backtracking, Stack |
| 739 | Daily Temperatures | Medium | ⬜ | Monotonic decreasing stack |
| 853 | Car Fleet | Medium | ⬜ | Sort + stack |
| 84 | Largest Rectangle in Histogram | Hard | ⬜ | Monotonic stack, Divide & conquer |

---

## Binary Search — 0 / 7 ⬜

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 704 | Binary Search | Easy | ⬜ | Classic iterative, Recursive |
| 74 | Search a 2D Matrix | Medium | ⬜ | Binary search on flattened index |
| 875 | Koko Eating Bananas | Medium | ⬜ | Binary search on answer |
| 153 | Find Minimum in Rotated Sorted Array | Medium | ⬜ | Modified binary search |
| 33 | Search in Rotated Sorted Array | Medium | ⬜ | Modified binary search |
| 981 | Time Based Key-Value Store | Medium | ⬜ | Binary search on timestamps |
| 4 | Median of Two Sorted Arrays | Hard | ⬜ | Binary search on partition |

---

## Linked List — 0 / 11 ⬜

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 206 | Reverse Linked List | Easy | ⬜ | Iterative, Recursive |
| 21 | Merge Two Sorted Lists | Easy | ⬜ | Iterative dummy node, Recursive |
| 141 | Linked List Cycle | Easy | ⬜ | Floyd's fast/slow pointers |
| 143 | Reorder List | Medium | ⬜ | Find middle + reverse + merge |
| 19 | Remove Nth Node From End | Medium | ⬜ | Two pointers with gap |
| 138 | Copy List with Random Pointer | Medium | ⬜ | Hash map, Interleaving |
| 2 | Add Two Numbers | Medium | ⬜ | Carry simulation |
| 287 | Find the Duplicate Number | Medium | ⬜ | Floyd's cycle detection, Binary search |
| 146 | LRU Cache | Medium | ⬜ | Hash map + doubly linked list |
| 23 | Merge K Sorted Lists | Hard | ⬜ | Min heap, Divide & conquer |
| 25 | Reverse Nodes in k-Group | Hard | ⬜ | Iterative group reversal |

---

## Trees — 0 / 15 ⬜

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 226 | Invert Binary Tree | Easy | ⬜ | Recursive DFS, Iterative BFS |
| 104 | Maximum Depth of Binary Tree | Easy | ⬜ | Recursive DFS, BFS level count |
| 543 | Diameter of Binary Tree | Easy | ⬜ | DFS with max-path tracking |
| 110 | Balanced Binary Tree | Easy | ⬜ | Bottom-up DFS with height |
| 100 | Same Tree | Easy | ⬜ | Recursive comparison |
| 572 | Subtree of Another Tree | Easy | ⬜ | DFS + same-tree check |
| 235 | Lowest Common Ancestor of BST | Medium | ⬜ | BST property traversal |
| 102 | Binary Tree Level Order Traversal | Medium | ⬜ | BFS queue |
| 199 | Binary Tree Right Side View | Medium | ⬜ | BFS last-per-level, DFS |
| 1448 | Count Good Nodes in Binary Tree | Medium | ⬜ | DFS with running max |
| 98 | Validate Binary Search Tree | Medium | ⬜ | DFS with min/max bounds |
| 230 | Kth Smallest Element in a BST | Medium | ⬜ | Inorder traversal |
| 105 | Construct Tree from Preorder & Inorder | Medium | ⬜ | Recursive divide + index map |
| 124 | Binary Tree Maximum Path Sum | Hard | ⬜ | DFS with global max |
| 297 | Serialize and Deserialize Binary Tree | Hard | ⬜ | BFS encode/decode, DFS |

---

## Graphs — 0 / 12 ⬜

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 200 | Number of Islands | Medium | ⬜ | BFS, DFS, Union-Find |
| 695 | Max Area of Island | Medium | ⬜ | DFS, BFS |
| 133 | Clone Graph | Medium | ⬜ | DFS + hash map, BFS |
| 417 | Pacific Atlantic Water Flow | Medium | ⬜ | Reverse BFS from both coasts |
| 130 | Surrounded Regions | Medium | ⬜ | BFS from border, DFS |
| 994 | Rotting Oranges | Medium | ⬜ | Multi-source BFS |
| 207 | Course Schedule | Medium | ⬜ | DFS cycle detection, Kahn's BFS |
| 210 | Course Schedule II | Medium | ⬜ | Topological sort (DFS + BFS) |
| 684 | Redundant Connection | Medium | ⬜ | Union-Find |
| 323 | Number of Connected Components | Medium | ⬜ | Union-Find, DFS |
| 127 | Word Ladder | Hard | ⬜ | BFS with character mutation |
| 332 | Reconstruct Itinerary | Hard | ⬜ | Hierholzer's algorithm |

---

## Heap / Priority Queue — 0 / 7 ⬜

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 703 | Kth Largest Element in a Stream | Easy | ⬜ | Min heap of size k |
| 1046 | Last Stone Weight | Easy | ⬜ | Max heap simulation |
| 973 | K Closest Points to Origin | Medium | ⬜ | Max heap, Quickselect |
| 215 | Kth Largest Element in an Array | Medium | ⬜ | Min heap, Quickselect |
| 621 | Task Scheduler | Medium | ⬜ | Greedy + max heap, Math |
| 355 | Design Twitter | Medium | ⬜ | Max heap per user merge |
| 295 | Find Median from Data Stream | Hard | ⬜ | Two heaps (max + min) |

---

## Backtracking — 0 / 9 ⬜

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 78 | Subsets | Medium | ⬜ | Backtrack, Bitmask |
| 39 | Combination Sum | Medium | ⬜ | Backtrack with pruning |
| 40 | Combination Sum II | Medium | ⬜ | Backtrack + sort + skip duplicates |
| 46 | Permutations | Medium | ⬜ | Backtrack with swap |
| 90 | Subsets II | Medium | ⬜ | Sort + skip duplicates |
| 79 | Word Search | Medium | ⬜ | DFS + visited marking |
| 131 | Palindrome Partitioning | Medium | ⬜ | Backtrack + palindrome check |
| 17 | Letter Combinations of Phone Number | Medium | ⬜ | Backtrack, Iterative BFS |
| 51 | N-Queens | Hard | ⬜ | Backtrack with column/diagonal sets |

---

## Tries — 0 / 3 ⬜

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 208 | Implement Trie | Medium | ⬜ | TrieNode class, array vs hash children |
| 211 | Design Add and Search Words | Medium | ⬜ | Trie + DFS wildcard search |
| 212 | Word Search II | Hard | ⬜ | Trie + DFS backtrack on board |

---

## Dynamic Programming — 0 / 13 ⬜

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 70 | Climbing Stairs | Easy | ⬜ | DP table, Two-variable (Fibonacci) |
| 746 | Min Cost Climbing Stairs | Easy | ⬜ | Bottom-up DP |
| 198 | House Robber | Medium | ⬜ | DP, Two-variable |
| 213 | House Robber II | Medium | ⬜ | Two-pass (skip first or last) |
| 5 | Longest Palindromic Substring | Medium | ⬜ | Expand around center, DP table |
| 647 | Palindromic Substrings | Medium | ⬜ | Expand around center, DP |
| 91 | Decode Ways | Medium | ⬜ | DP with one/two digit checks |
| 322 | Coin Change | Medium | ⬜ | Bottom-up DP, BFS |
| 152 | Maximum Product Subarray | Medium | ⬜ | Track min/max running product |
| 139 | Word Break | Medium | ⬜ | DP + set lookup |
| 300 | Longest Increasing Subsequence | Medium | ⬜ | DP O(n²), Patience sort O(n log n) |
| 416 | Partition Equal Subset Sum | Medium | ⬜ | 0/1 knapsack DP |
| 1143 | Longest Common Subsequence | Medium | ⬜ | 2D DP table |

---

## Greedy — 0 / 8 ⬜

| # | Title | Difficulty | Status | Approaches |
|---|-------|------------|--------|------------|
| 53 | Maximum Subarray | Medium | ⬜ | Kadane's algorithm, Divide & conquer |
| 55 | Jump Game | Medium | ⬜ | Greedy max-reach |
| 45 | Jump Game II | Medium | ⬜ | Greedy farthest jump |
| 134 | Gas Station | Medium | ⬜ | One-pass net balance |
| 846 | Hand of Straights | Medium | ⬜ | Greedy sort + ordered map |
| 763 | Partition Labels | Medium | ⬜ | Last-occurrence greedy |
| 678 | Valid Parenthesis String | Medium | ⬜ | Greedy lo/hi range tracking |
| 1899 | Merge Triplets to Form Target Triplet | Medium | ⬜ | Greedy filter + max merge |

---

## Progress Summary

| Topic | Done | Total | % |
|-------|------|-------|---|
| Arrays & Hashing | 15 | 15 | 100% |
| Two Pointers | 5 | 5 | 100% |
| Sliding Window | 0 | 6 | 0% |
| Stack | 0 | 7 | 0% |
| Binary Search | 0 | 7 | 0% |
| Linked List | 0 | 11 | 0% |
| Trees | 0 | 15 | 0% |
| Graphs | 0 | 12 | 0% |
| Heap / Priority Queue | 0 | 7 | 0% |
| Backtracking | 0 | 9 | 0% |
| Tries | 0 | 3 | 0% |
| Dynamic Programming | 0 | 13 | 0% |
| Greedy | 0 | 8 | 0% |
| **Total** | **20** | **119** | **17%** |
