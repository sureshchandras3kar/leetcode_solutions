# AI Reference — Index

Context files for AI assistants working on this project. Load the file(s) relevant to your task.

| File | Load when you need to… |
|------|------------------------|
| [project-structure.md](./project-structure.md) | Understand the repo layout, tech stack, build system |
| [problem-page-guide.md](./problem-page-guide.md) | Write or edit a problem MDX page |
| [components.md](./components.md) | Use any custom Astro component (ComplexityBadges, LogicPlayer, etc.) |
| [solution-files.md](./solution-files.md) | Write solution code files in any of the 6 languages |
| [css-reference.md](./css-reference.md) | Add or edit CSS, use existing utility classes |
| [sidebar-navigation.md](./sidebar-navigation.md) | Add a problem to sidebar, set prev/next links |
| [mdx-gotchas.md](./mdx-gotchas.md) | Debug MDX build errors, understand formatting rules |

## Quick facts

- **Framework**: Astro + Starlight (static site, GitHub Pages)
- **Base URL**: `/leetcode_solutions`
- **Content root**: `src/content/docs/`
- **Problem pages**: `src/content/docs/problems/{slug}.mdx`
- **Solution files**: `solutions/{Language}/{slug}_{approach}.{ext}`
- **6 languages**: Python, C++, Java, JavaScript, Rust, Golang
- **Build**: `npm run build` → `dist/`
- **Dev**: `npm run dev`
- **Progress**: see `docs/problems-status.md`
