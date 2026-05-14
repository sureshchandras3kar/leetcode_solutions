# Sidebar Navigation

How the sidebar is configured and how to add new problems.

---

## Config file

All sidebar config lives in `astro.config.mjs` inside the `starlight({ sidebar: [...] })` array.

---

## Sidebar structure

```javascript
sidebar: [
  // Top-level link (no children)
  { label: 'Study Plans', slug: 'study-plans' },

  // Group with children
  {
    label: 'Problems',          // Group heading (shown as uppercase label)
    items: [ /* entries */ ],
  },

  // Another topic group
  {
    label: 'Two Pointers',
    items: [ /* entries */ ],
  },
]
```

---

## Problem entry format

Each problem is a sidebar item object:

```javascript
{
  label: 'Group Anagrams',
  slug: 'problems/group-anagrams',
  badge: { text: '#49 · Medium', variant: 'note' },
  attrs: {
    'data-lc-problem':    '49',
    'data-lc-difficulty': 'medium',
    'data-lc-title':      'Group Anagrams',
    'data-lc-tags':       'array,hash table,string,sorting',
  },
},
```

### Badge variants

| Difficulty | `variant` | Color |
|------------|-----------|-------|
| Easy | `'success'` | Green |
| Medium | `'note'` | Blue/accent |
| Hard | `'danger'` | Red |

Badge text format: `'#{number} · {Difficulty}'` — note the `·` (middle dot, U+00B7), not a hyphen.

### `data-lc-*` attributes

These are read by client-side JS in `public/animations.js` to enable the filtering/search UI on the All Problems page.

| Attribute | Value format |
|-----------|-------------|
| `data-lc-problem` | Problem number as string: `'49'` |
| `data-lc-difficulty` | Lowercase: `'easy'`, `'medium'`, `'hard'` |
| `data-lc-title` | Full problem title |
| `data-lc-tags` | Comma-separated lowercase LeetCode tag names |

Tag names follow LeetCode conventions (all lowercase, space-separated multi-word tags like `'hash table'`, `'two pointers'`).

---

## Adding a new problem to the sidebar

1. Decide which group it belongs to (e.g., `'Sliding Window'`).
2. If the group doesn't exist yet, add a new `{ label: 'Sliding Window', items: [] }` object.
3. Add the problem entry to the group's `items` array.
4. Set `badge.variant` based on difficulty.
5. Fill all four `data-lc-*` attrs.

### New group template
```javascript
{
  label: 'Sliding Window',
  items: [
    {
      label: 'Best Time to Buy and Sell Stock',
      slug: 'problems/best-time-to-buy-and-sell-stock',
      badge: { text: '#121 · Easy', variant: 'success' },
      attrs: {
        'data-lc-problem':    '121',
        'data-lc-difficulty': 'easy',
        'data-lc-title':      'Best Time to Buy and Sell Stock',
        'data-lc-tags':       'array,dynamic programming',
      },
    },
  ],
},
```

---

## prev / next navigation

Each MDX problem page has `prev` and `next` in frontmatter that renders navigation arrows at the bottom of the page.

### Rules
- `prev: false` → no back arrow (first problem in chain)
- `next: false` → no forward arrow (last problem in chain)
- Use relative paths with trailing slash: `"../two-sum/"` not `"/problems/two-sum"`

### Format
```yaml
prev:
  label: "#349 Intersection of Two Arrays"
  link: "../intersection-of-two-arrays/"
next:
  label: "#49 Group Anagrams"
  link: "../group-anagrams/"
```

Label format: `"#{number} {Title}"` — matches how Starlight displays nav links.

### Current chain (Arrays & Hashing)

```
Two Sum (#1)
  → Palindrome Number (#9)
  → Majority Element (#169)
  → Contains Duplicate (#217)
  → Valid Anagram (#242)
  → Missing Number (#268)
  → Intersection of Two Arrays (#349)
  → Group Anagrams (#49)
  → Top K Frequent Elements (#347)
  → Product of Array Except Self (#238)
  → Valid Sudoku (#36)
  → Longest Consecutive Sequence (#128)
  → Subarray Sum Equals K (#560)
  → Contiguous Array (#525)
  → First Missing Positive (#41) [next: false]
```

### Current chain (Two Pointers)

```
Valid Palindrome (#125) [prev: false]
  → Two Sum II (#167)
  → 3Sum (#15)
  → Container With Most Water (#11)
  → Trapping Rain Water (#42) [next: false]
```

---

## When adding a new topic group

1. Add the group in `astro.config.mjs` after the last existing group.
2. Set `prev: false` on the first problem in the group.
3. Set `next: false` on the last problem in the group.
4. Wire up prev/next within the chain.
5. Do NOT link between groups in prev/next (each topic is its own chain).

---

## Slug conventions

- Sidebar `slug` must match the MDX file path relative to `src/content/docs/`: `'problems/group-anagrams'` → `src/content/docs/problems/group-anagrams.mdx`
- Slugs are kebab-case of the problem title
- The `slug` in `astro.config.mjs` has no leading slash and no `.mdx` extension

---

## All Problems listing page

`src/content/docs/problems/index.mdx` — slug `'problems'`. This page renders a filterable table of all problems using the `data-lc-*` attrs from sidebar items. Do not add navigation (prev/next) to this page.
