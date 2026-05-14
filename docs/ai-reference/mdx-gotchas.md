# MDX Gotchas

Common MDX build errors and how to avoid them. Any error stops the entire build.

---

## Curly braces `{}` in table cells

**Error:** `Unexpected character {` or JSX parse error  
**Cause:** MDX treats `{...}` as a JSX expression anywhere in the document, including inside Markdown table cells.

**BAD — build breaks:**
```markdown
| Step | Map state |
| --- | --- |
| 1 | {0:-1} |
| 2 | {0:-1, 1:2} |
```

**GOOD — wrap in backticks:**
```markdown
| Step | Map state |
| --- | --- |
| 1 | `{0:-1}` |
| 2 | `{0:-1, 1:2}` |
```

Any `{` that would start a JSX expression must be inside backticks, inside a fenced code block, or inside `{`...`}` as a JS string expression (but that's rarely needed).

---

## Angle brackets `<` and `>` in table cells

**Error:** `Unexpected token` or JSX tag parse error  
**Cause:** `<` followed by a word starts a JSX element. `>=`, `<=` trigger the same issue.

**BAD:**
```markdown
| Condition | Result |
| --- | --- |
| left <= right | continue |
| height > max | update max |
```

**GOOD — use HTML entities:**
```markdown
| Condition | Result |
| --- | --- |
| left &lt;= right | continue |
| height &gt; max | update max |
```

HTML entities to remember:
| Character | Entity |
|-----------|--------|
| `<` | `&lt;` |
| `>` | `&gt;` |
| `<=` | `&lt;=` |
| `>=` | `&gt;=` |
| `&` | `&amp;` |
| `"` | `&quot;` (in attribute values) |

---

## Curly braces in inline code outside tables

Inside prose (not a table), backtick-quoted code is fine: `` `{seen}` ``. The problem is only bare `{` outside backticks.

---

## JSX props with `{` in LogicPlayer / component props

When passing arrays or objects as component props, the outer `{...}` is a valid JSX expression delimiter. Inner braces inside the data structure are fine:

```mdx
<LogicPlayer
  steps={[
    { title: 'Step 1', memory: ['key → value'] }
  ]}
/>
```

The outer `{[...]}` is JSX; inner `{ title: ... }` is a JS object literal — both are fine because they're parsed as JavaScript, not Markdown.

---

## Backtick escaping inside code in JSX props

If you need a backtick inside a string in a JSX prop value, use a template literal or escape it. In practice, just avoid backticks inside `steps={[...]}` strings.

---

## HTML comments in MDX

Standard HTML comments `<!-- -->` do NOT work in MDX. Use JSX comments instead:

```mdx
{/* This is a comment */}
```

---

## Raw `import` strings and special characters

When importing solution files as `?raw`, the content is treated as a raw string — no issues. But if you manually write a code string inline in JSX props, watch for `{`, `\`, and `<`.

---

## Frontmatter values

- `tags` must be an array: `tags: [Array, Hash Table]` — no quotes needed for simple words; use quotes for multi-word: `tags: ["Hash Table", "Two Pointers"]`
- `difficulty` must be exactly `Easy`, `Medium`, or `Hard` — capitalized first letter only
- `prev: false` and `next: false` are valid (disable the nav link)
- `prev`/`next` with `link:` must use relative paths with trailing slash: `"../two-sum/"`

---

## `template: splash` pages

Pages using `template: splash` (study-plans, home, roadmaps) render without the standard sidebar + content column layout. All content must be raw HTML/MDX. Standard Markdown tables and prose render but look unstyled — use CSS classes for layout instead.

---

## Import block position

The import block **must come immediately after the closing `---` of frontmatter**, before any content. If you put a blank line before the imports, some MDX parsers may mis-identify them as content.

**Correct:**
```mdx
---
title: Two Sum
---
import { Aside } from '@astrojs/starlight/components';

## Problem Statement
```

---

## `@solutions` alias in imports

`@solutions` resolves to `./solutions/` from the project root. The alias is configured in `astro.config.mjs` via `vite.resolve.alias`. Always use `?raw` suffix to import as a string:

```typescript
import pyHashMap from '@solutions/Python/two_sum_hash_map.py?raw';
```

Without `?raw`, Vite tries to process the file as a module and fails.

---

## Heading levels in TOC

Starlight's TOC is configured to show h2 and h3 only (`minHeadingLevel: 2, maxHeadingLevel: 3`). Use `##` for main sections, `###` for subsections within an approach. `####` and deeper are invisible in the TOC.

---

## Aside inside ordered lists

Putting an `<Aside>` directly inside an ordered list item (`1. ... <Aside>`) can break MDX parsing. Place Asides at the top level, outside of lists.

---

## Line breaks in JSX props

Whitespace (including newlines) between JSX attributes is fine. But don't put bare text between component tags and their props — it gets interpreted as children.

---

## Common build error checklist

When `npm run build` fails on an MDX file:

1. Search for bare `{` in table cells — wrap in backticks
2. Search for `<` followed by text in table cells — use `&lt;`
3. Search for `>` in table cells (comparisons) — use `&gt;`
4. Check that all JSX component props are closed (`/>` or `</Component>`)
5. Check that `steps={[...]}` arrays have matching brackets
6. Check frontmatter types: `difficulty` capitalization, `tags` as an array
