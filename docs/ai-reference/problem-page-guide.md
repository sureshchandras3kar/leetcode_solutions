# Problem Page Guide

Complete reference for writing a new problem MDX page. Follow this exactly.

---

## File location

`src/content/docs/problems/{kebab-case-slug}.mdx`

Slug rule: LeetCode title in kebab-case. Examples:
- "Two Sum" → `two-sum.mdx`
- "Group Anagrams" → `group-anagrams.mdx`
- "Trapping Rain Water" → `trapping-rain-water.mdx`

---

## Frontmatter

```yaml
---
title: Group Anagrams
description: Group strings that are anagrams of each other. Two approaches — sort key O(n·k·log k) and character count O(n·k) — in six languages.
leetcodeNumber: 49
difficulty: Medium
leetcodeSlug: group-anagrams
tags: [Array, Hash Table, String, Sorting]
prev:
  label: "#349 Intersection of Two Arrays"
  link: "../intersection-of-two-arrays/"
next:
  label: "#347 Top K Frequent Elements"
  link: "../top-k-frequent-elements/"
---
```

**Rules:**
- `description`: one sentence — state the problem goal, list approach names with complexities, end with "in six languages"
- `difficulty`: exactly `Easy`, `Medium`, or `Hard` (capital first letter)
- `leetcodeSlug`: from the LeetCode URL, kebab-case (e.g. `https://leetcode.com/problems/group-anagrams/` → `group-anagrams`)
- `tags`: Title Case, matches LeetCode tag names
- `prev`/`next` links use relative paths with trailing slash: `"../problem-slug/"`
- First problem in chain: `prev: false`
- Last problem in chain: `next: false`

---

## Imports block

Always placed immediately after frontmatter, before any content:

```typescript
import { Aside, Tabs, TabItem, Code } from '@astrojs/starlight/components';
import ComplexityBadges from '../../../components/ComplexityBadges.astro';
import ApproachTag from '../../../components/ApproachTag.astro';
import LogicPlayer from '../../../components/LogicPlayer.astro';
import RelatedProblems from '../../../components/RelatedProblems.astro';

import pyApproach1   from '@solutions/Python/slug_approach1.py?raw';
import cppApproach1  from '@solutions/C++/slug_approach1.cpp?raw';
import javaApproach1 from '@solutions/Java/slug_approach1.java?raw';
import jsApproach1   from '@solutions/JavaScript/slug_approach1.js?raw';
import rustApproach1 from '@solutions/Rust/slug_approach1.rs?raw';
import goApproach1   from '@solutions/Golang/slug_approach1.go?raw';

import pyApproach2   from '@solutions/Python/slug_approach2.py?raw';
// ... repeat for all approaches × 6 languages
```

Variable naming convention: `{languageAbbrev}{ApproachPascalCase}`  
e.g., `pyHashMap`, `cppBruteForce`, `javaCharCount`, `rustTwoPtr`, `goSortKey`

---

## Page section order

Every problem page must contain these sections in this order:

### 1. Problem Statement

```markdown
## Problem Statement

Given an integer array `nums`...

<Aside type="tip" title="What to notice first">
- Key observation 1
- Key observation 2  
- The fastest approach uses X
</Aside>
```

### 2. Examples table

```markdown
### Examples

| Input | Output | Explanation |
| --- | --- | --- |
| `[2, 7, 11, 15]`, target `9` | `[0, 1]` | `nums[0] + nums[1] = 9` |
```

### 3. Constraints

```markdown
## Constraints

- `2 <= nums.length <= 10^4`
- `-10^9 <= nums[i] <= 10^9`
```

### 4. Real-World Applications

```markdown
## Real-World Applications

<Aside type="note" title="Where this pattern appears">
- **Use case 1** — explanation
- **Use case 2** — explanation
- **Use case 3** — explanation
- **Use case 4** — explanation
</Aside>
```

### 5. Complexity Comparison table

```markdown
## Complexity Comparison

| Approach | Time | Space | Best When |
| --- | --- | --- | --- |
| Brute Force | O(n²) | O(1) | Array is tiny |
| Hash Map | O(n) | O(n) | General case |
```

### 6. Approach quick-select cards (HTML)

```html
<div class="lc-approach-quick">
  <div class="lc-approach-quick__card lc-approach-quick__card--best">
    <div class="lc-approach-quick__label">Best For Speed</div>
    <div class="lc-approach-quick__name">Hash Map</div>
    <div class="lc-approach-quick__stats">O(n) time &middot; O(n) space</div>
  </div>
  <div class="lc-approach-quick__card">
    <div class="lc-approach-quick__label">Sorted Input</div>
    <div class="lc-approach-quick__name">Two-Pointer</div>
    <div class="lc-approach-quick__stats">O(n log n) time &middot; O(n) space</div>
  </div>
</div>
```

Only the best/recommended card gets `lc-approach-quick__card--best`.

### 7. One section per approach

Repeat this block for each approach:

```markdown
## Approach 1: Hash Map (Recommended)

<ApproachTag tag="recommended" />

Explanation paragraph...

<ComplexityBadges time="O(n)" space="O(n)" timeColor="green" spaceColor="yellow" timeNote="Single pass" spaceNote="Hash map storage" />

### Step-by-step Example

**Input:** `nums = [2, 7, 11, 15]`, `target = 9`

| Step | i | num | complement | seen | Action |
| --- | --- | --- | --- | --- | --- |
| 1 | 0 | 2 | 7 | `{}` | Store `{2:0}` |
| 2 | 1 | 7 | 2 | `{2:0}` | Found → return `[0,1]` |

<LogicPlayer
  title="How the hash map finds the answer"
  description="Use Next or Autoplay to watch each step."
  target="9"
  steps={[
    {
      title: 'Start with empty map',
      note: 'No numbers seen yet.',
      formula: 'seen = {}',
      callout: 'We have no memory yet.',
      items: [
        { label: '2', state: 'default', index: 0 },
        { label: '7', state: 'default', index: 1 },
      ],
      memory: [],
    },
    {
      title: 'Check 2',
      note: 'Need 7, not in map. Store 2.',
      formula: 'target - nums[0] = 9 - 2 = 7',
      callout: 'No match yet.',
      items: [
        { label: '2', state: 'active', index: 0 },
        { label: '7', state: 'default', index: 1 },
      ],
      memory: ['2 → index 0'],
    },
  ]}
/>

### Pseudocode

```plaintext
function solve(input):
    ...
```

### Solution Code

<Tabs>
  <TabItem label="Python">
    <Code code={pyHashMap} lang="python" title="two_sum_hash_map.py" />
  </TabItem>
  <TabItem label="C++">
    <Code code={cppHashMap} lang="cpp" title="two_sum_hash_map.cpp" />
  </TabItem>
  <TabItem label="Java">
    <Code code={javaHashMap} lang="java" title="two_sum_hash_map.java" />
  </TabItem>
  <TabItem label="JavaScript">
    <Code code={jsHashMap} lang="javascript" title="two_sum_hash_map.js" />
  </TabItem>
  <TabItem label="Rust">
    <Code code={rustHashMap} lang="rust" title="two_sum_hash_map.rs" />
  </TabItem>
  <TabItem label="Go">
    <Code code={goHashMap} lang="go" title="two_sum_hash_map.go" />
  </TabItem>
</Tabs>
```

### 8. Common mistakes

```markdown
## Common mistakes

<Aside type="caution" title="Watch for these pitfalls">
- Mistake 1
- Mistake 2
- Mistake 3
</Aside>
```

### 9. RelatedProblems

```typescript
<RelatedProblems problems={[
  { number: 217, title: 'Contains Duplicate', difficulty: 'Easy', slug: 'problems/contains-duplicate', leetcodeSlug: 'contains-duplicate' },
  { number: 15, title: '3Sum', difficulty: 'Medium', leetcodeSlug: '3sum' },
]} />
```

Use `slug` only when the problem already has a page in this site.  
Without `slug`, links go to `leetcode.com/problems/{leetcodeSlug}/`.

### 10. Interview Tips

```markdown
## Interview Tips

<Aside type="tip" title="Key trade-offs to discuss">
- **Why X over Y?** Trade-off explanation.
- **Edge cases to mention:** ...
- **Follow-up:** ...
</Aside>
```

---

## Aside types

| type | Color | Use for |
|------|-------|---------|
| `tip` | Green | "What to notice first", Interview tips |
| `note` | Blue/accent | Real-world applications |
| `caution` | Orange | Common mistakes, warnings |
| `danger` | Red | Critical constraints, breaking changes |

---

## LogicPlayer step schema

```typescript
{
  title: string;       // Step heading (shown in bold)
  note: string;        // Paragraph explanation
  formula?: string;    // Monospace code line (e.g. "target - nums[i] = 9 - 2 = 7")
  callout?: string;    // Emphasized insight (shown if non-empty)
  items?: Array<{
    label: string;     // Value shown in chip
    state?: 'default' | 'active' | 'match';  // Visual state
    index?: number;    // Index label shown above chip
  }>;
  memory?: string[];   // Chips shown in "MEMORY / LOOKUP STATE" panel
                       // Empty array → shows "Empty" placeholder
}
```

State colors:
- `default` → neutral background
- `active` → orange (#f59e0b) — currently being processed
- `match` → green (#10b981) — result found

Autoplay interval: 1400ms per step.

---

## Complexity color guide

| Complexity | timeColor/spaceColor |
|------------|---------------------|
| O(1) | `green` |
| O(log n) | `green` |
| O(n) | `green` |
| O(n log n) | `yellow` |
| O(n²) | `red` |
| O(n³) | `red` |
| O(n) space when trade-off | `yellow` |

---

## ApproachTag variants

| tag | Label | Color |
|-----|-------|-------|
| `recommended` | ★ Recommended | Purple (accent) |
| `interview` | 🎯 Interview Favourite | Orange |
| `space-optimal` | 💾 Space-Optimal | Green |
| `simple` | ✓ Simple | Gray |
