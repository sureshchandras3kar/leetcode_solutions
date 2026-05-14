# Components Reference

All custom Astro components live in `src/components/`. Import paths in MDX are relative to the file.

---

## ComplexityBadges

Renders Time and Space complexity as colored chips.

### Import
```typescript
import ComplexityBadges from '../../../components/ComplexityBadges.astro';
```

### Props

| Prop | Type | Required | Default | Notes |
|------|------|----------|---------|-------|
| `time` | `string` | ✓ | — | e.g. `"O(n)"`, `"O(n log n)"` |
| `space` | `string` | ✓ | — | e.g. `"O(1)"`, `"O(n)"` |
| `timeColor` | `'green' \| 'yellow' \| 'red'` | | `'green'` | |
| `spaceColor` | `'green' \| 'yellow' \| 'red'` | | `'green'` | |
| `timeNote` | `string` | | — | Small note shown inside the time chip |
| `spaceNote` | `string` | | — | Small note shown inside the space chip |

### Color guide

| Complexity | Color |
|-----------|-------|
| O(1), O(log n), O(n) | `green` |
| O(n log n) | `yellow` |
| O(n) space when a trade-off | `yellow` |
| O(n²), O(n³) | `red` |

### Usage
```mdx
<ComplexityBadges time="O(n)" space="O(n)" timeColor="green" spaceColor="yellow" timeNote="Single pass" spaceNote="Hash map storage" />
```

---

## ApproachTag

Renders a small colored label tag at the start of an approach section.

### Import
```typescript
import ApproachTag from '../../../components/ApproachTag.astro';
```

### Props

| Prop | Type | Required |
|------|------|----------|
| `tag` | `'recommended' \| 'interview' \| 'space-optimal' \| 'simple'` | ✓ |

### Tag variants

| `tag` | Label | Color |
|-------|-------|-------|
| `recommended` | ★ Recommended | Purple (accent) |
| `interview` | 🎯 Interview Favourite | Orange |
| `space-optimal` | 💾 Space-Optimal | Green |
| `simple` | ✓ Simple | Gray |

### Usage
```mdx
## Approach 1: Hash Map (Recommended)

<ApproachTag tag="recommended" />
```

Place immediately after the `## Approach N:` heading, before the explanation paragraph.

---

## LogicPlayer

Interactive step-by-step algorithm visualizer. Renders "Animated walkthrough" panel with Back / Autoplay / Next / Reset controls.

### Import
```typescript
import LogicPlayer from '../../../components/LogicPlayer.astro';
```

### Props

| Prop | Type | Required | Notes |
|------|------|----------|-------|
| `title` | `string` | ✓ | Heading shown in bold |
| `description` | `string` | | Paragraph below title |
| `target` | `string` | | Displayed as "Target: {value}" badge |
| `steps` | `Step[]` | ✓ | Array of step objects (see schema below) |

### Step schema

```typescript
{
  title: string;       // Step heading (shown in bold in note panel)
  note: string;        // Paragraph explanation
  formula?: string;    // Monospace line (e.g. "target - nums[i] = 9 - 2 = 7")
  callout?: string;    // Emphasized insight line (shown when non-empty)
  items?: Array<{
    label: string;              // Value shown in chip
    state?: 'default' | 'active' | 'match';  // Visual highlight
    index?: number;             // Index label shown above chip
  }>;
  memory?: string[];   // Chips in "Memory / lookup state" panel
                       // Empty array [] → shows "Empty" placeholder
}
```

### Item states

| state | Color | Use for |
|-------|-------|---------|
| `default` | Neutral | Unprocessed element |
| `active` | Orange (#f59e0b) | Currently being examined |
| `match` | Green (#10b981) | Found / result element |

### Behavior
- Autoplay interval: 1400 ms per step
- Progress bar advances as steps play
- `data-steps` is serialized to JSON and parsed client-side by `public/animations.js`

### Usage example
```mdx
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
```

### Placement
Put `LogicPlayer` after the "Step-by-step Example" table and before "Pseudocode".

---

## RelatedProblems

Renders a "Related Problems" section with internal or external links, difficulty pills, and problem numbers.

### Import
```typescript
import RelatedProblems from '../../../components/RelatedProblems.astro';
```

### Props

| Prop | Type | Required |
|------|------|----------|
| `problems` | `Problem[]` | ✓ |

### Problem object

```typescript
{
  number: number;          // LeetCode problem number
  title: string;           // Display name
  difficulty: 'Easy' | 'Medium' | 'Hard';
  slug?: string;           // Internal doc path — use when site page exists
  leetcodeSlug: string;    // From LeetCode URL, always required
}
```

- If `slug` is provided → link goes to `/leetcode_solutions/{slug}/`
- If `slug` is omitted → link goes to `https://leetcode.com/problems/{leetcodeSlug}/` (opens in new tab with ↗ indicator)

### Usage
```mdx
<RelatedProblems problems={[
  { number: 217, title: 'Contains Duplicate', difficulty: 'Easy', slug: 'problems/contains-duplicate', leetcodeSlug: 'contains-duplicate' },
  { number: 15,  title: '3Sum',               difficulty: 'Medium', leetcodeSlug: '3sum' },
]} />
```

Place in section 9 (after Common Mistakes, before Interview Tips).

---

## Mermaid

Renders a Mermaid diagram. Theme-aware: switches between dark and light Mermaid themes automatically.

### Import
```typescript
import Mermaid from '../../../components/Mermaid.astro';
```

### Props

| Prop | Type | Required |
|------|------|----------|
| `chart` | `string` | ✓ |

### Usage
```mdx
<Mermaid chart={`
flowchart LR
  A[Input] --> B{Hash map lookup}
  B -- hit --> C[Return indices]
  B -- miss --> D[Store in map]
  D --> A
`} />
```

Mermaid is initialized via CDN script in `astro.config.mjs`. A `MutationObserver` re-renders diagrams when the user switches between dark/light mode.

---

## PageTitle (Starlight override)

Lives at `src/components/overrides/PageTitle.astro`. Registered in `astro.config.mjs`:
```javascript
components: {
  PageTitle: './src/components/overrides/PageTitle.astro',
}
```

**Do NOT import or call this manually.** It wraps Starlight's default `PageTitle` and auto-injects problem metadata (number badge, difficulty pill, tag chips, LeetCode link) on any page that has `leetcodeNumber` in frontmatter. No `<ProblemMeta>` call needed in MDX.

---

## Starlight built-ins

These are imported from `@astrojs/starlight/components` — no local path needed.

```typescript
import { Aside, Tabs, TabItem, Code } from '@astrojs/starlight/components';
```

### Aside

| `type` | Color | Use for |
|--------|-------|---------|
| `tip` | Green | "What to notice first", Interview Tips |
| `note` | Blue/accent | Real-World Applications |
| `caution` | Orange | Common Mistakes, warnings |
| `danger` | Red | Critical constraints |

```mdx
<Aside type="caution" title="Watch for these pitfalls">
- Mistake 1
- Mistake 2
</Aside>
```

### Tabs / TabItem + Code

Always use this exact tab order: Python, C++, Java, JavaScript, Rust, Go.

```mdx
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

- `code` prop: the `?raw` import variable
- `lang` prop: lowercase language identifier
- `title` prop: shown in the code block header bar — use the actual filename
