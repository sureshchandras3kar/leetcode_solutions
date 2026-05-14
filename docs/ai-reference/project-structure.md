# Project Structure

## Tech Stack

| Layer | Tool |
|-------|------|
| Framework | Astro 6 + `@astrojs/starlight` |
| Content | MDX files in `src/content/docs/` |
| Styling | Starlight defaults + `src/styles/custom.css` (≈2200 lines) |
| Interactivity | `public/animations.js` (vanilla JS, ≈830 lines) |
| Code display | `@astrojs/starlight` Expressive Code (built-in) |
| Diagrams | Mermaid (CDN, theme-aware) |
| Fonts | Inter + JetBrains Mono (Google Fonts) |
| Deploy | GitHub Actions → GitHub Pages |
| Build output | `dist/` (static HTML) |

## Directory Layout

```
leetcode_solutions/
├── astro.config.mjs          # Starlight config, sidebar, head scripts
├── src/
│   ├── assets/
│   │   └── programmer.png    # Hero image on home page
│   ├── components/
│   │   ├── ApproachTag.astro
│   │   ├── ComplexityBadges.astro
│   │   ├── LogicPlayer.astro
│   │   ├── Mermaid.astro
│   │   ├── ProblemMeta.astro
│   │   ├── RelatedProblems.astro
│   │   └── overrides/
│   │       └── PageTitle.astro   # Starlight override — auto-injects ProblemMeta
│   ├── content/
│   │   └── docs/
│   │       ├── index.mdx             # Home page (template: splash)
│   │       ├── study-plans.mdx       # Study Plans page (template: splash)
│   │       ├── problems/
│   │       │   ├── index.mdx         # All Problems listing
│   │       │   ├── two-sum.mdx
│   │       │   └── ...              # One .mdx per problem
│   │       └── roadmaps/
│   │           └── arrays_hashing.mdx
│   ├── content.config.ts     # Zod schema extending Starlight's docsSchema
│   └── styles/
│       └── custom.css        # All custom CSS (design tokens, components, animations)
├── solutions/                # Raw solution code imported as ?raw strings
│   ├── Python/
│   ├── C++/
│   ├── Java/
│   ├── JavaScript/
│   ├── Rust/
│   └── Golang/
├── public/
│   ├── favicon.ico
│   └── animations.js         # All client-side interactivity
├── docs/
│   ├── problems-status.md    # Completion tracker (20/119)
│   └── ai-reference/         # ← you are here
└── .github/
    └── workflows/
        └── build.yml         # CI: npm ci → npm run build → deploy Pages
```

## Content Schema (`src/content.config.ts`)

Extends Starlight's built-in schema with:

```typescript
leetcodeNumber: z.number().optional()      // e.g. 1, 49, 42
difficulty:     z.enum(['Easy','Medium','Hard']).optional()
leetcodeSlug:   z.string().optional()      // e.g. "two-sum"
tags:           z.array(z.string()).optional() // e.g. ["Array","Hash Table"]
```

## Vite Alias

`@solutions` → `./solutions/`  
Used in MDX imports: `import py from '@solutions/Python/two_sum_hash_map.py?raw'`

## Key Config Facts

- **Base path**: `/leetcode_solutions` (GitHub Pages repo subpath)
- **Last updated**: enabled via `lastUpdated: true` (reads git timestamps)
- **CI requirement**: `actions/checkout` must use `fetch-depth: 0` for timestamps to work
- **Syntax themes**: `github-dark-dimmed` (dark) / `github-light` (light)
- **TOC levels**: h2 and h3 only (`minHeadingLevel: 2, maxHeadingLevel: 3`)
- **PageTitle override**: `./src/components/overrides/PageTitle.astro` — auto-injects problem meta on any page that has `leetcodeNumber` in frontmatter

## Build

```bash
npm run dev      # dev server (respects base path)
npm run build    # static build → dist/
npm run preview  # preview built site
```

Build currently produces **25 pages**. Any MDX parse error stops the entire build.
