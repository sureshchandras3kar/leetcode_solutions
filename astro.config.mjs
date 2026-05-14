import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import { fileURLToPath } from 'url';

export default defineConfig({
  site: 'https://sureshchandras3kar.github.io',
  base: '/leetcode_solutions',
  integrations: [
    starlight({
      title: 'LeetCode Solutions',
      favicon: '/favicon.ico',

      /* ── Last-updated timestamp from git ── */
      lastUpdated: true,

      /* ── "Edit this page" link on every doc ── */
      editLink: {
        baseUrl: 'https://github.com/sureshchandras3kar/leetcode_solutions/edit/main/',
      },

      /* ── TOC: show only h2 by default (less noise) ── */
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },

      /* ── PageTitle override: injects ProblemMeta from frontmatter ── */
      components: {
        PageTitle: './src/components/overrides/PageTitle.astro',
      },

      social: [
        { icon: 'github',   label: 'GitHub',    href: 'https://github.com/sureshchandras3kar' },
        { icon: 'linkedin', label: 'LinkedIn',   href: 'https://www.linkedin.com/in/suresh-chandrasekar/' },
        { icon: 'x.com',    label: 'X / Twitter', href: 'https://twitter.com/ChennaiProgramr' },
      ],

      /* ── Sidebar with native badges ── */
      sidebar: [
        {
          label: 'Problems',
          items: [
            {
              label: 'All Problems',
              slug: 'problems',
            },
            {
              label: 'Two Sum',
              slug: 'problems/two-sum',
              badge: { text: '#1 · Easy', variant: 'success' },
              attrs: { 'data-lc-problem': '1', 'data-lc-difficulty': 'easy', 'data-lc-title': 'Two Sum', 'data-lc-tags': 'array,hash table,two pointers' },
            },
            {
              label: 'Palindrome Number',
              slug: 'problems/palindrome-number',
              badge: { text: '#9 · Easy', variant: 'success' },
              attrs: { 'data-lc-problem': '9', 'data-lc-difficulty': 'easy', 'data-lc-title': 'Palindrome Number', 'data-lc-tags': 'math,two pointers' },
            },
            {
              label: 'Majority Element',
              slug: 'problems/majority-element',
              badge: { text: '#169 · Easy', variant: 'success' },
              attrs: { 'data-lc-problem': '169', 'data-lc-difficulty': 'easy', 'data-lc-title': 'Majority Element', 'data-lc-tags': 'array,hash table,counting' },
            },
            {
              label: 'Contains Duplicate',
              slug: 'problems/contains-duplicate',
              badge: { text: '#217 · Easy', variant: 'success' },
              attrs: { 'data-lc-problem': '217', 'data-lc-difficulty': 'easy', 'data-lc-title': 'Contains Duplicate', 'data-lc-tags': 'array,hash table,sorting' },
            },
            {
              label: 'Valid Anagram',
              slug: 'problems/valid-anagram',
              badge: { text: '#242 · Easy', variant: 'success' },
              attrs: { 'data-lc-problem': '242', 'data-lc-difficulty': 'easy', 'data-lc-title': 'Valid Anagram', 'data-lc-tags': 'hash table,string,sorting' },
            },
            {
              label: 'Missing Number',
              slug: 'problems/missing-number',
              badge: { text: '#268 · Easy', variant: 'success' },
              attrs: { 'data-lc-problem': '268', 'data-lc-difficulty': 'easy', 'data-lc-title': 'Missing Number', 'data-lc-tags': 'array,hash table,math' },
            },
            {
              label: 'Intersection of Two Arrays',
              slug: 'problems/intersection-of-two-arrays',
              badge: { text: '#349 · Easy', variant: 'success' },
              attrs: { 'data-lc-problem': '349', 'data-lc-difficulty': 'easy', 'data-lc-title': 'Intersection of Two Arrays', 'data-lc-tags': 'array,hash table,two pointers' },
            },
            {
              label: 'Group Anagrams',
              slug: 'problems/group-anagrams',
              badge: { text: '#49 · Medium', variant: 'note' },
              attrs: { 'data-lc-problem': '49', 'data-lc-difficulty': 'medium', 'data-lc-title': 'Group Anagrams', 'data-lc-tags': 'array,hash table,string,sorting' },
            },
            {
              label: 'Top K Frequent Elements',
              slug: 'problems/top-k-frequent-elements',
              badge: { text: '#347 · Medium', variant: 'note' },
              attrs: { 'data-lc-problem': '347', 'data-lc-difficulty': 'medium', 'data-lc-title': 'Top K Frequent Elements', 'data-lc-tags': 'array,hash table,sorting,heap,bucket sort' },
            },
            {
              label: 'Product of Array Except Self',
              slug: 'problems/product-except-self',
              badge: { text: '#238 · Medium', variant: 'note' },
              attrs: { 'data-lc-problem': '238', 'data-lc-difficulty': 'medium', 'data-lc-title': 'Product of Array Except Self', 'data-lc-tags': 'array,prefix sum' },
            },
            {
              label: 'Valid Sudoku',
              slug: 'problems/valid-sudoku',
              badge: { text: '#36 · Medium', variant: 'note' },
              attrs: { 'data-lc-problem': '36', 'data-lc-difficulty': 'medium', 'data-lc-title': 'Valid Sudoku', 'data-lc-tags': 'array,hash table,matrix' },
            },
            {
              label: 'Longest Consecutive Sequence',
              slug: 'problems/longest-consecutive-sequence',
              badge: { text: '#128 · Medium', variant: 'note' },
              attrs: { 'data-lc-problem': '128', 'data-lc-difficulty': 'medium', 'data-lc-title': 'Longest Consecutive Sequence', 'data-lc-tags': 'array,hash table,union find' },
            },
            {
              label: 'Subarray Sum Equals K',
              slug: 'problems/subarray-sum-equals-k',
              badge: { text: '#560 · Medium', variant: 'note' },
              attrs: { 'data-lc-problem': '560', 'data-lc-difficulty': 'medium', 'data-lc-title': 'Subarray Sum Equals K', 'data-lc-tags': 'array,hash table,prefix sum' },
            },
            {
              label: 'Contiguous Array',
              slug: 'problems/contiguous-array',
              badge: { text: '#525 · Medium', variant: 'note' },
              attrs: { 'data-lc-problem': '525', 'data-lc-difficulty': 'medium', 'data-lc-title': 'Contiguous Array', 'data-lc-tags': 'array,hash table,prefix sum' },
            },
            {
              label: 'First Missing Positive',
              slug: 'problems/first-missing-positive',
              badge: { text: '#41 · Hard', variant: 'danger' },
              attrs: { 'data-lc-problem': '41', 'data-lc-difficulty': 'hard', 'data-lc-title': 'First Missing Positive', 'data-lc-tags': 'array,hash table' },
            },
          ],
        },
        {
          label: 'Two Pointers',
          items: [
            {
              label: 'Valid Palindrome',
              slug: 'problems/valid-palindrome',
              badge: { text: '#125 · Easy', variant: 'success' },
              attrs: { 'data-lc-problem': '125', 'data-lc-difficulty': 'easy', 'data-lc-title': 'Valid Palindrome', 'data-lc-tags': 'two pointers,string' },
            },
            {
              label: 'Two Sum II',
              slug: 'problems/two-sum-ii',
              badge: { text: '#167 · Medium', variant: 'note' },
              attrs: { 'data-lc-problem': '167', 'data-lc-difficulty': 'medium', 'data-lc-title': 'Two Sum II', 'data-lc-tags': 'array,two pointers,binary search' },
            },
            {
              label: '3Sum',
              slug: 'problems/three-sum',
              badge: { text: '#15 · Medium', variant: 'note' },
              attrs: { 'data-lc-problem': '15', 'data-lc-difficulty': 'medium', 'data-lc-title': '3Sum', 'data-lc-tags': 'array,two pointers,sorting' },
            },
            {
              label: 'Container With Most Water',
              slug: 'problems/container-with-most-water',
              badge: { text: '#11 · Medium', variant: 'note' },
              attrs: { 'data-lc-problem': '11', 'data-lc-difficulty': 'medium', 'data-lc-title': 'Container With Most Water', 'data-lc-tags': 'array,two pointers,greedy' },
            },
            {
              label: 'Trapping Rain Water',
              slug: 'problems/trapping-rain-water',
              badge: { text: '#42 · Hard', variant: 'danger' },
              attrs: { 'data-lc-problem': '42', 'data-lc-difficulty': 'hard', 'data-lc-title': 'Trapping Rain Water', 'data-lc-tags': 'array,two pointers,dynamic programming,stack' },
            },
          ],
        },
      ],

      expressiveCode: {
        themes: ['github-dark-dimmed', 'github-light'],
      },

      head: [
        { tag: 'link', attrs: { rel: 'preconnect', href: 'https://fonts.googleapis.com' } },
        { tag: 'link', attrs: { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' } },
        {
          tag: 'link',
          attrs: {
            rel: 'stylesheet',
            href: 'https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,400;0,14..32,500;0,14..32,600;0,14..32,700&family=JetBrains+Mono:wght@400;500&display=swap',
          },
        },
        {
          tag: 'script',
          attrs: { src: 'https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js', defer: true },
        },
        {
          tag: 'script',
          content: `
            (function () {
              var _observer;

              function renderMermaid() {
                var isDark = document.documentElement.getAttribute('data-theme') === 'dark';
                var darkVars = {
                  primaryColor: '#312e81',
                  primaryTextColor: '#e0e7ff',
                  primaryBorderColor: '#818cf8',
                  lineColor: '#a78bfa',
                  secondaryColor: '#1e1b4b',
                  tertiaryColor: '#0f0e23',
                  background: '#1e1b4b',
                  mainBkg: '#2d2b55',
                  nodeBorder: '#818cf8',
                  clusterBkg: '#1e1b4b',
                  titleColor: '#e0e7ff',
                  edgeLabelBackground: '#1e1b4b',
                  fontFamily: 'Inter, system-ui, sans-serif',
                };
                mermaid.initialize({
                  startOnLoad: false,
                  theme: isDark ? 'dark' : 'default',
                  fontSize: 15,
                  flowchart: { curve: 'basis', padding: 24 },
                  themeVariables: isDark ? darkVars : { fontFamily: 'Inter, system-ui, sans-serif' },
                });
                var nodes = document.querySelectorAll('.mermaid');
                nodes.forEach(function (el) {
                  var src = el.getAttribute('data-mermaid-src');
                  if (src) {
                    el.innerHTML = src;
                    el.removeAttribute('data-processed');
                  }
                });
                if (nodes.length) mermaid.run({ nodes: nodes });
              }

              function waitForMermaidAndRender() {
                if (window.mermaid) {
                  renderMermaid();
                  if (!_observer) {
                    _observer = new MutationObserver(function (muts) {
                      if (muts.some(function (m) { return m.attributeName === 'data-theme'; })) renderMermaid();
                    });
                    _observer.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
                  }
                } else {
                  setTimeout(waitForMermaidAndRender, 80);
                }
              }

              document.addEventListener('astro:page-load', waitForMermaidAndRender);
            })();
          `,
        },
        {
          tag: 'script',
          attrs: { src: '/leetcode_solutions/animations.js', defer: true },
        },
      ],
      customCss: ['./src/styles/custom.css'],
    }),
  ],
  vite: {
    resolve: {
      alias: {
        '@solutions': fileURLToPath(new URL('./solutions', import.meta.url)),
      },
    },
  },
});
