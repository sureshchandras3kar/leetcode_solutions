# CSS Reference

All custom styles are in `src/styles/custom.css` (~2200 lines). Classes use two prefixes:
- `.lc-*` — problem page components
- `.sp-*` — study plans page components

---

## Design Tokens (CSS variables)

```css
/* Light mode */
--sl-color-accent-low:  #e0e7ff;
--sl-color-accent:      #4f46e5;
--sl-color-accent-high: #3730a3;

--lc-grad:        linear-gradient(135deg, #4f46e5 0%, #7c3aed 55%, #a21caf 100%);
--lc-grad-subtle: linear-gradient(135deg, /* 15% opacity accent */ );

--lc-radius-lg: 0.875rem;
--lc-radius-md: 0.625rem;
--lc-shadow-accent: 0 0 0 1px color-mix(in srgb, var(--sl-color-accent) 20%, ...);
--sl-content-width: 46rem;   /* prose width ~75 chars */
```

```css
/* Dark mode — [data-theme='dark'] */
--sl-color-accent-low:  #1e1b4b;
--sl-color-accent:      #818cf8;
--sl-color-accent-high: #c7d2fe;
--lc-grad: linear-gradient(135deg, #818cf8 0%, #a78bfa 55%, #e879f9 100%);
```

Use `var(--sl-color-accent)` etc. for any new component that should respect the theme.

---

## Animation Keyframes

| Name | Effect |
|------|--------|
| `lc-fadein` | Fade up from 10px below |
| `lc-gradient-shift` | Animated gradient background position |
| `lc-shimmer` | Horizontal shimmer sweep |
| `lc-pulse-ring` | Pulsing box-shadow ring |

Usage: `animation: lc-fadein 0.35s ease;`

---

## Problem Page Components (`.lc-*`)

### Problem Meta (auto-injected by PageTitle override)

```css
.lc-problem-meta          /* container row */
.lc-problem-meta__number  /* #42 badge */
.lc-problem-meta__diff    /* difficulty pill */
.lc-problem-meta__diff--easy    /* green */
.lc-problem-meta__diff--medium  /* orange/yellow */
.lc-problem-meta__diff--hard    /* red */
.lc-problem-meta__tag     /* topic tag chip */
.lc-problem-meta__link    /* "▶ Try on LeetCode" link */
```

### Complexity Badges

```css
.lc-complexity-row        /* flex row containing both badges */
.lc-badge                 /* base chip */
.lc-badge--green          /* time/space green */
.lc-badge--yellow         /* time/space yellow */
.lc-badge--red            /* time/space red */
.lc-badge__label          /* "⏱ Time" / "💾 Space" */
.lc-badge__value          /* "O(n)" */
.lc-badge__note           /* optional sub-note */
```

### Approach Tag

```css
.lc-approach-tag                    /* base pill */
.lc-approach-tag--recommended       /* purple/accent */
.lc-approach-tag--interview         /* orange */
.lc-approach-tag--space-optimal     /* green */
.lc-approach-tag--simple            /* gray */
```

### Approach Quick-Select Cards

HTML structure used in MDX:
```html
<div class="lc-approach-quick">
  <div class="lc-approach-quick__card lc-approach-quick__card--best">
    <div class="lc-approach-quick__label">Best For Speed</div>
    <div class="lc-approach-quick__name">Hash Map</div>
    <div class="lc-approach-quick__stats">O(n) time &middot; O(n) space</div>
  </div>
  <div class="lc-approach-quick__card">
    ...
  </div>
</div>
```

```css
.lc-approach-quick              /* grid container */
.lc-approach-quick__card        /* card base */
.lc-approach-quick__card--best  /* highlighted card (accent border, subtle bg) */
.lc-approach-quick__label       /* "Best For Speed" small label */
.lc-approach-quick__name        /* "Hash Map" approach name */
.lc-approach-quick__stats       /* "O(n) time · O(n) space" stats line */
```

### Logic Player

```css
.lc-logic-player                /* section container */
.lc-logic-player__header        /* title + controls row */
.lc-logic-player__eyebrow       /* "Animated walkthrough" small label */
.lc-logic-player__title         /* main title */
.lc-logic-player__description   /* subtitle */
.lc-logic-player__controls      /* button row */
.lc-logic-player__button        /* base button */
.lc-logic-player__button--ghost /* "Autoplay" secondary style */
.lc-logic-player__button--primary /* "Next" primary style */
.lc-logic-player__meta          /* step count + progress row */
.lc-logic-player__meta-row      /* inner flex row */
.lc-logic-player__step-count    /* "Step 1 of 5" */
.lc-logic-player__target        /* "Target: 9" badge */
.lc-logic-player__progress      /* progress bar track */
.lc-logic-player__formula       /* monospace formula line */
.lc-logic-player__panel         /* two-column: array + memory */
.lc-logic-player__panel-label   /* "Array state" / "Memory / lookup state" */
.lc-logic-player__items         /* array chips row */
.lc-logic-player__memory        /* memory chips row */
.lc-logic-player__note          /* step title + note + callout */
.lc-logic-player__callout       /* italic callout line */
```

Item chip classes (applied by animations.js):
```css
.lc-item                 /* base chip */
.lc-item--active         /* orange — currently processing */
.lc-item--match          /* green — found/result */
.lc-item__index          /* index label above chip */
.lc-memory-chip          /* memory panel chip */
```

### Related Problems

```css
.lc-related               /* container */
.lc-related__heading      /* "Related Problems" h3 */
.lc-related__list         /* ul */
.lc-related__item         /* li — flex row */
.lc-related__title        /* internal link */
.lc-related__title--ext   /* external link (LeetCode) */
.lc-related__num          /* "#42" problem number */
```

---

## Study Plans Page Components (`.sp-*`)

### Featured Gradient Cards

```css
.sp-featured-grid         /* 2-col responsive grid */
.sp-card                  /* base card */
.sp-card__title           /* card title */
.sp-card__meta            /* subtitle / count */
.sp-card__cta             /* call-to-action link */
.sp-grad--indigo          /* indigo → purple gradient */
.sp-grad--teal            /* teal → cyan gradient */
.sp-grad--orange          /* orange → red gradient */
.sp-grad--violet          /* violet → pink gradient */
```

### Concept (Problem List) Cards

```css
.sp-concept-grid               /* responsive card grid */
.sp-concept-card               /* base card (gray, coming soon) */
.sp-concept-card.sp-concept--active  /* active card (accent border, white bg) */
.sp-concept-header             /* title + count row */
.sp-concept-bar                /* progress bar track */
.sp-concept-fill               /* progress bar fill (set width% inline) */
.sp-problem-list               /* ul of problem items */
.sp-problem                    /* li — single problem row */
.sp-problem--solved            /* solved variant (check mark green) */
.sp-problem-check              /* ✓ or ○ icon */
.sp-problem-link               /* problem anchor */
.sp-problem-num                /* "#1" number span */
.sp-problem-title              /* "Two Sum" title span */
.sp-diff-pip                   /* difficulty pill */
.sp-pip--easy                  /* green */
.sp-pip--medium                /* orange */
.sp-pip--hard                  /* red */
.sp-concept-viewall            /* "View all N problems →" link */
.sp-ghost-line                 /* skeleton placeholder line (coming soon cards) */
```

### Difficulty Cards

```css
.sp-diff-grid             /* 3-col responsive grid */
.sp-diff-card             /* difficulty tier card */
.sp-diff-card--easy       /* green accent */
.sp-diff-card--medium     /* orange accent */
.sp-diff-card--hard       /* red accent */
```

### Challenge Cards

```css
.sp-challenge-grid        /* 2-col responsive grid */
.sp-challenge-card        /* challenge card */
```

---

## Utility / Global

### Heading decorations

`h2` inside `.sl-markdown-content` automatically gets a left gradient bar (`::before` pseudo-element) and a bottom border.

`h3` inside `.sl-markdown-content` gets a thinner left accent bar.

These styles are automatic — do not add classes to headings.

### Step-by-step table

Standard Markdown tables in problem pages are automatically styled with:
- Alternating row shading
- Sticky header
- Accent border on first column

No extra class needed.

### Active sidebar link

`.sidebar-content a[aria-current='page']` gets a gradient left border and subtle background — automatic via Starlight's active-page attribute.

---

## Adding new CSS

1. Add to `src/styles/custom.css` at the end.
2. Use `.lc-` prefix for problem-page components, `.sp-` for study-plans.
3. Prefer `color-mix(in srgb, var(--sl-color-accent) X%, ...)` for theme-aware tinting.
4. Use `var(--lc-radius-md)` / `var(--lc-radius-lg)` for border radii.
5. Never override Starlight internals with `!important` unless matching an existing pattern in the file.
