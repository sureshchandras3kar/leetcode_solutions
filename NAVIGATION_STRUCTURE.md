# LeetCode Solutions - Complete Navigation Structure

## Overview
This document describes the canonical ordering and navigation structure for all 141 LeetCode problems in the repository.

## Statistics
- **Total Problems**: 141
- **LeetCode Number Range**: 1 - 918
- **First Problem**: #1 (Two Sum)
- **Last Problem**: #918 (Maximum Sum Circular Subarray)

## Structure Files

### 1. `navigation_structure.json`
JSON format containing all 141 problems with their navigation metadata:
```json
{
  "leetcodeNumber": 1,
  "slug": "two-sum",
  "title": "Two Sum",
  "prev": {
    "number": null,
    "slug": null,
    "title": null
  },
  "next": {
    "number": 2,
    "slug": "add-two-numbers",
    "title": "Add Two Numbers"
  }
}
```

### 2. `navigation_structure.csv`
CSV format for easy import/reference with columns:
- LeetCode Number
- Problem Slug
- Problem Title
- Previous Number
- Previous Slug
- Previous Title
- Next Number
- Next Slug
- Next Title

## Navigation Rules

### Canonical Ordering
Problems are ordered by their LeetCode problem number (ascending, 1-918). This ordering determines all prev/next relationships.

### Edge Cases
- **Problem #1 (Two Sum)**: No previous problem (prev = null)
- **Problem #918 (Maximum Sum Circular Subarray)**: No next problem (next = null)

### Gaps in LeetCode Numbering
Not all numbers 1-918 have corresponding problems. Notable gaps:
- #6-8 (3 numbers missing)
- #10, #12-14, #16, #18 (various single gaps)
- Large gap: #452-501 (49 numbers missing)
- Large gap: #503-559 (57 numbers missing)
- Large gap: #561-636 (76 numbers missing)
- Large gap: #638-908 (271 numbers missing)

## Usage for Frontmatter Updates

Each MDX file in `src/content/docs/problems/` should have prev/next fields updated as follows:

### For Problem with Both Prev and Next
```yaml
prev:
  label: "#4 Median of Two Sorted Arrays"
  link: "../median-sorted-arrays/"
next:
  label: "#9 Palindrome Number"
  link: "../palindrome-number/"
```

### For First Problem (#1)
```yaml
prev: false
next:
  label: "#2 Add Two Numbers"
  link: "../add-two-numbers/"
```

### For Last Problem (#918)
```yaml
prev:
  label: "#909 Snakes and Ladders"
  link: "../snakes-and-ladders/"
next: false
```

## Complete Problem List

See the generated tables in `navigation_structure.json` and `navigation_structure.csv` for the complete list of all 141 problems with their navigation metadata.

## Next Steps

This navigation structure should be used by subsequent agents to:
1. Update all 141 MDX files with consistent prev/next frontmatter
2. Validate that all navigation links are bidirectional and correct
3. Generate navigation components for the UI
