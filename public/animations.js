(function () {
  /* ── Reading Progress Bar ─────────────────────────────────── */
  function createProgressBar() {
    let bar = document.getElementById('lc-progress');
    if (!bar) {
      bar = document.createElement('div');
      bar.id = 'lc-progress';
      document.body.prepend(bar);
    }
    const update = () => {
      const max = document.documentElement.scrollHeight - window.innerHeight;
      bar.style.width = max > 0 ? `${(window.scrollY / max) * 100}%` : '0%';
    };
    if (window._lcProgressHandler) window.removeEventListener('scroll', window._lcProgressHandler);
    window._lcProgressHandler = update;
    window.addEventListener('scroll', update, { passive: true });
    update();
  }

  /* ── Page Fade-in ─────────────────────────────────────────── */
  function pageTransition() {
    const main = document.querySelector('.sl-markdown-content');
    if (!main) return;
    main.classList.remove('lc-page-in');
    void main.offsetWidth;
    main.classList.add('lc-page-in');
  }

  /* ── Section Reveal on Scroll ─────────────────────────────── */
  function initSectionReveal() {
    const els = document.querySelectorAll(
      '.sl-markdown-content h2, .sl-markdown-content h3, .sl-markdown-content .starlight-aside'
    );
    if (!els.length) return;

    const io = new IntersectionObserver(
      (entries) => {
        entries.forEach((e) => {
          if (e.isIntersecting) {
            e.target.classList.add('lc-visible');
            io.unobserve(e.target);
          }
        });
      },
      { threshold: 0.12, rootMargin: '0px 0px -40px 0px' }
    );

    els.forEach((el) => {
      el.classList.remove('lc-visible');
      el.classList.add('lc-reveal');
      io.observe(el);
    });
  }

  /* ── Card Stagger Entrance ────────────────────────────────── */
  function initCardStagger() {
    const cards = [...document.querySelectorAll('.card')];
    if (!cards.length) return;

    const io = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            const idx = cards.indexOf(entry.target);
            setTimeout(() => entry.target.classList.add('lc-visible'), idx * 100);
            io.unobserve(entry.target);
          }
        });
      },
      { threshold: 0.05 }
    );

    cards.forEach((c) => {
      c.classList.remove('lc-visible');
      c.classList.add('lc-card-reveal');
      io.observe(c);
    });
  }

  /* ── Card Tilt 3D ─────────────────────────────────────────── */
  function initCardTilt() {
    document.querySelectorAll('.card').forEach((card) => {
      card.addEventListener('mousemove', (e) => {
        const r = card.getBoundingClientRect();
        const x = (e.clientX - r.left) / r.width - 0.5;
        const y = (e.clientY - r.top) / r.height - 0.5;
        card.style.transform = `perspective(700px) rotateY(${x * 10}deg) rotateX(${-y * 10}deg) translateY(-5px)`;
        card.style.transition = 'transform 0.08s linear';
      });
      card.addEventListener('mouseleave', () => {
        card.style.transform = '';
        card.style.transition = 'transform 0.45s ease, box-shadow 0.22s ease, border-color 0.22s ease';
      });
    });
  }

  /* ── Back to Top ──────────────────────────────────────────── */
  function createBackToTop() {
    let btn = document.getElementById('lc-back-top');
    if (!btn) {
      btn = document.createElement('button');
      btn.id = 'lc-back-top';
      btn.setAttribute('aria-label', 'Back to top');
      btn.innerHTML =
        '<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="18 15 12 9 6 15"/></svg>';
      document.body.appendChild(btn);
      btn.addEventListener('click', () => window.scrollTo({ top: 0, behavior: 'smooth' }));
    }

    if (window._lcBackTopHandler) window.removeEventListener('scroll', window._lcBackTopHandler);
    window._lcBackTopHandler = () => btn.classList.toggle('lc-visible', window.scrollY > 350);
    window.addEventListener('scroll', window._lcBackTopHandler, { passive: true });
    window._lcBackTopHandler();
  }

  /* ── Heading Anchor Links ─────────────────────────────────── */
  function initHeadingAnchors() {
    document
      .querySelectorAll('.sl-markdown-content h2[id], .sl-markdown-content h3[id]')
      .forEach((h) => {
        if (h.querySelector('.lc-anchor')) return;
        const a = document.createElement('a');
        a.href = `#${h.id}`;
        a.className = 'lc-anchor';
        a.setAttribute('aria-hidden', 'true');
        a.setAttribute('tabindex', '-1');
        a.textContent = '#';
        h.appendChild(a);
      });
  }

  /* ── Mermaid Fade-in + Scale ──────────────────────────────── */
  function initMermaidAnimation() {
    const watchMermaid = (container) => {
      const mo = new MutationObserver(() => {
        const svg = container.querySelector('svg:not([data-lc-anim])');
        if (!svg) return;
        svg.setAttribute('data-lc-anim', '1');
        svg.style.opacity = '0';
        svg.style.transform = 'scale(0.96)';
        requestAnimationFrame(() => {
          svg.style.transition = 'opacity 0.5s ease, transform 0.5s ease';
          svg.style.opacity = '1';
          svg.style.transform = 'scale(1)';
        });
        mo.disconnect();
      });
      mo.observe(container, { childList: true, subtree: true });
    };

    document.querySelectorAll('.mermaid').forEach(watchMermaid);
  }

  /* ── Tab Switch Crossfade ─────────────────────────────────── */
  function initTabCrossfade() {
    document.querySelectorAll('[role="tab"]').forEach((tab) => {
      tab.addEventListener('click', () => {
        const panelId = tab.getAttribute('aria-controls');
        const panel = panelId && document.getElementById(panelId);
        if (!panel) return;
        panel.classList.remove('lc-tab-in');
        void panel.offsetWidth;
        panel.classList.add('lc-tab-in');
      });
    });
  }

  /* ── Inline code shimmer on hover ────────────────────────── */
  function initCodeHover() {
    document.querySelectorAll('.sl-markdown-content :not(pre) > code').forEach((el) => {
      el.classList.add('lc-code');
    });
  }

  /* ── Sticky Approach Navigator ───────────────────────────── */
  function initApproachNav() {
    const approaches = [...document.querySelectorAll('.sl-markdown-content h2[id]')]
      .filter((h) => /^approach\s+\d+/i.test(h.textContent.trim()));
    if (approaches.length < 2) return;

    let nav = document.getElementById('lc-approach-nav');
    if (!nav) {
      nav = document.createElement('nav');
      nav.id = 'lc-approach-nav';
      nav.setAttribute('aria-label', 'Jump to approach');
      document.body.appendChild(nav);
    }
    nav.innerHTML = '';

    approaches.forEach((h, i) => {
      const raw = h.textContent.replace(/#$/, '').trim();
      const match = raw.match(/Approach\s*\d+[:\s]+(.+)/i);
      const label = match ? match[1].replace(/\(.*?\)/g, '').trim() : `Approach ${i + 1}`;
      const btn = document.createElement('button');
      btn.className = 'lc-approach-nav__item';
      btn.textContent = `${i + 1}. ${label}`;
      btn.addEventListener('click', () => h.scrollIntoView({ behavior: 'smooth', block: 'start' }));
      nav.appendChild(btn);
    });

    function activateApproach(index) {
      if (index < 0 || index >= approaches.length) return;
      approaches[index].scrollIntoView({ behavior: 'smooth', block: 'start' });
    }

    // Show nav once the first approach heading scrolls out of view
    new IntersectionObserver(
      ([e]) => nav.classList.toggle('lc-approach-nav--visible', !e.isIntersecting),
      { threshold: 0, rootMargin: '-80px 0px 0px 0px' }
    ).observe(approaches[0]);

    // Highlight which approach is actively in the viewport
    const activeObs = new IntersectionObserver(
      (entries) => {
        entries.forEach((e) => {
          if (e.isIntersecting) {
            const idx = approaches.indexOf(e.target);
            nav.querySelectorAll('.lc-approach-nav__item').forEach((btn, i) =>
              btn.classList.toggle('lc-approach-nav__item--active', i === idx)
            );
          }
        });
      },
      { threshold: 0, rootMargin: '-80px 0px -60% 0px' }
    );
    approaches.forEach((h) => activeObs.observe(h));

    if (window._lcApproachNavKeyHandler) {
      document.removeEventListener('keydown', window._lcApproachNavKeyHandler);
    }

    window._lcApproachNavKeyHandler = (event) => {
      const target = event.target;
      if (
        target instanceof HTMLElement &&
        (target.isContentEditable || /INPUT|TEXTAREA|SELECT/.test(target.tagName))
      ) {
        return;
      }

      const key = event.key;
      if (/^[1-9]$/.test(key)) {
        const index = Number(key) - 1;
        if (index < approaches.length) {
          event.preventDefault();
          activateApproach(index);
        }
        return;
      }

      if (key !== 'ArrowLeft' && key !== 'ArrowRight') return;

      const buttons = [...nav.querySelectorAll('.lc-approach-nav__item')];
      if (!buttons.length) return;

      const activeIndex = buttons.findIndex((btn) => btn.classList.contains('lc-approach-nav__item--active'));
      const fallbackIndex = activeIndex >= 0 ? activeIndex : 0;
      const delta = key === 'ArrowRight' ? 1 : -1;
      const nextIndex = Math.max(0, Math.min(approaches.length - 1, fallbackIndex + delta));

      if (nextIndex !== fallbackIndex) {
        event.preventDefault();
        activateApproach(nextIndex);
      }
    };

    document.addEventListener('keydown', window._lcApproachNavKeyHandler);
  }

  /* ── Mermaid Fullscreen Overlay ──────────────────────────── */
  function initMermaidFullscreen() {
    document.querySelectorAll('.mermaid').forEach((container) => {
      if (container.dataset.lcZoom) return;
      container.dataset.lcZoom = '1';
      container.style.cursor = 'zoom-in';
      container.title = 'Click to expand';

      container.addEventListener('click', () => {
        const svg = container.querySelector('svg');
        if (!svg) return;

        const overlay = document.createElement('div');
        overlay.id = 'lc-mermaid-overlay';
        overlay.innerHTML = `
          <div class="lc-mermaid-overlay__inner">
            <button class="lc-mermaid-overlay__close" aria-label="Close fullscreen">&#x2715;</button>
            ${svg.outerHTML}
          </div>`;
        document.body.appendChild(overlay);
        document.body.style.overflow = 'hidden';
        requestAnimationFrame(() => overlay.classList.add('lc-mermaid-overlay--visible'));

        const close = () => {
          overlay.classList.remove('lc-mermaid-overlay--visible');
          setTimeout(() => { overlay.remove(); document.body.style.overflow = ''; }, 250);
        };
        overlay.addEventListener('click', (e) => { if (e.target === overlay) close(); });
        overlay.querySelector('.lc-mermaid-overlay__close').addEventListener('click', close);
        const onKey = (e) => {
          if (e.key === 'Escape') { close(); document.removeEventListener('keydown', onKey); }
        };
        document.addEventListener('keydown', onKey);
      });
    });
  }

  /* ── Copy Button Feedback ────────────────────────────────── */
  function initCopyFeedback() {
    function showCopyTip(host) {
      const existing = host.querySelector('.lc-copy-tip');
      if (existing) existing.remove();
      const tip = document.createElement('span');
      tip.className = 'lc-copy-tip';
      tip.textContent = '\u2713 Copied!';
      host.appendChild(tip);
      requestAnimationFrame(() => tip.classList.add('lc-copy-tip--visible'));
      setTimeout(() => {
        tip.classList.remove('lc-copy-tip--visible');
        setTimeout(() => tip.remove(), 200);
      }, 1600);
    }

    async function copyText(text) {
      try {
        await navigator.clipboard.writeText(text);
        return true;
      } catch {
        const ta = document.createElement('textarea');
        ta.value = text;
        ta.setAttribute('readonly', '');
        ta.style.position = 'fixed';
        ta.style.opacity = '0';
        document.body.appendChild(ta);
        ta.select();
        const copied = document.execCommand('copy');
        ta.remove();
        return copied;
      }
    }

    // Add our own copy button for code blocks that don't provide one.
    document.querySelectorAll('.sl-markdown-content pre').forEach((pre) => {
      if (!(pre instanceof HTMLElement) || pre.dataset.lcCopyBound) return;
      if (pre.closest('[data-logic-player]')) return;

      const code = pre.querySelector('code');
      if (!code) return;

      pre.dataset.lcCopyBound = '1';
      pre.classList.add('lc-code-copy-host');

      const button = document.createElement('button');
      button.type = 'button';
      button.className = 'lc-code-copy-btn';
      button.textContent = 'Copy';
      button.setAttribute('aria-label', 'Copy code to clipboard');
      button.addEventListener('click', async () => {
        const ok = await copyText(code.innerText);
        if (ok) showCopyTip(button);
      });
      pre.appendChild(button);
    });

    // Preserve feedback on any built-in copy buttons provided by renderers.
    document.querySelectorAll('.expressive-code button').forEach((btn) => {
      if (btn.dataset.lcCopy) return;
      btn.dataset.lcCopy = '1';
      btn.style.position = 'relative';
      btn.addEventListener('click', () => showCopyTip(btn));
    });
  }

  /* ── Roadmap Checklist Persistence ───────────────────────── */
  function initRoadmapChecklistPersistence() {
    const checklist = document.querySelector('[data-roadmap-checklist]');
    if (!checklist) return;

    const STORAGE_KEY = 'lc-roadmap-checklist-v1';
    const progressEl = checklist.querySelector('[data-roadmap-progress]');
    const resetButton = checklist.querySelector('[data-roadmap-reset]');
    const boxes = [...checklist.querySelectorAll('input[type="checkbox"][data-roadmap-id]')];
    if (!boxes.length) return;

    let state = {};
    try {
      state = JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}') || {};
    } catch {
      state = {};
    }

    function refreshProgress() {
      const complete = boxes.filter((box) => box.checked).length;
      if (progressEl) progressEl.textContent = `${complete}/${boxes.length} complete`;
    }

    boxes.forEach((box) => {
      const id = box.dataset.roadmapId;
      if (!id) return;
      if (Object.prototype.hasOwnProperty.call(state, id)) {
        box.checked = Boolean(state[id]);
      }
      box.addEventListener('change', () => {
        state[id] = box.checked;
        localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
        refreshProgress();
      });
    });

    if (resetButton && !resetButton.dataset.lcResetBound) {
      resetButton.dataset.lcResetBound = '1';
      resetButton.addEventListener('click', () => {
        state = {};
        boxes.forEach((box) => {
          box.checked = false;
        });
        localStorage.removeItem(STORAGE_KEY);
        refreshProgress();
      });
    }

    refreshProgress();
  }

  /* ── Language Tab Persistence ─────────────────────── */
  function initLangPersistence() {
    const STORAGE_KEY = 'lc-preferred-lang';

    function getLang() { try { return localStorage.getItem(STORAGE_KEY); } catch { return null; } }
    function setLang(l) { try { localStorage.setItem(STORAGE_KEY, l); } catch {} }

    // Restore saved language across all tab groups on the page
    function restoreLang(lang) {
      document.querySelectorAll('[role="tab"]').forEach((tab) => {
        if (tab.textContent.trim() === lang) tab.click();
      });
    }

    // Save language when user explicitly clicks a tab
    document.querySelectorAll('[role="tab"]').forEach((tab) => {
      tab.addEventListener('click', () => setLang(tab.textContent.trim()));
    });

    const saved = getLang();
    if (saved) restoreLang(saved);
  }

  /* ── Mobile Collapsible Approach Sections ────────────── */
  function initMobileAccordion() {
    if (window.innerWidth > 900) return; // desktop: leave default layout
    const approaches = document.querySelectorAll('.sl-markdown-content h2[id]');
    approaches.forEach((h) => {
      if (!/approach/i.test(h.textContent)) return;
      // Gather all siblings until the next h2
      const siblings = [];
      let el = h.nextElementSibling;
      while (el && el.tagName !== 'H2') {
        siblings.push(el);
        el = el.nextElementSibling;
      }
      if (!siblings.length) return;

      // Wrap siblings in a collapsible container
      const wrapper = document.createElement('div');
      wrapper.className = 'lc-accordion-body';
      h.after(wrapper);
      siblings.forEach((s) => wrapper.appendChild(s));

      // Toggle button indicator on h2
      const indicator = document.createElement('span');
      indicator.className = 'lc-accordion-indicator';
      indicator.textContent = '\u25B2';
      h.appendChild(indicator);
      h.classList.add('lc-accordion-header');

      // First approach open by default, rest collapsed
      const isFirst = !h.previousElementSibling?.classList.contains('lc-accordion-header');
      if (!isFirst) {
        wrapper.classList.add('lc-accordion-body--closed');
        indicator.style.transform = 'rotate(180deg)';
      }

      h.addEventListener('click', () => {
        const closed = wrapper.classList.toggle('lc-accordion-body--closed');
        indicator.style.transform = closed ? 'rotate(180deg)' : '';
      });
    });
  }

  /* ── Sidebar Problem Count Chip ───────────────────── */
  function initSidebarCount() {
    document.querySelectorAll('.group-label').forEach((label) => {
      const group = label.closest('li') || label.parentElement;
      const count = group ? group.querySelectorAll('a').length : 0;
      if (!count || label.querySelector('.lc-count-chip')) return;
      const chip = document.createElement('span');
      chip.className = 'lc-count-chip';
      chip.textContent = count;
      label.querySelector('span')?.appendChild(chip);
    });
  }

  /* ── Sidebar Difficulty Filter + Sort ───────────────── */
  function initSidebarProblemControls() {
    const navRoot = document.querySelector('.sidebar-content');
    if (!navRoot) return;

    const problemLinks = [...navRoot.querySelectorAll('a[data-lc-problem][data-lc-difficulty]')];
    if (!problemLinks.length) return;

    const STORAGE_KEY = 'lc-sidebar-problem-controls-v2';

    const firstItem = problemLinks[0].closest('li');
    const parentList = firstItem && firstItem.parentElement;
    if (!firstItem || !parentList) return;

    let controls = navRoot.querySelector('[data-sidebar-problem-controls]');
    if (!controls) {
      controls = document.createElement('div');
      controls.className = 'lc-sidebar-controls';
      controls.setAttribute('data-sidebar-problem-controls', '1');
      controls.innerHTML = `
        <label class="lc-sidebar-controls__label" for="lc-sidebar-filter">Difficulty</label>
        <select id="lc-sidebar-filter" class="lc-sidebar-controls__select" data-sidebar-filter>
          <option value="all">All</option>
          <option value="easy">Easy</option>
          <option value="medium">Medium</option>
          <option value="hard">Hard</option>
        </select>
        <label class="lc-sidebar-controls__label" for="lc-sidebar-tag">Tag</label>
        <select id="lc-sidebar-tag" class="lc-sidebar-controls__select" data-sidebar-tag>
          <option value="all">All tags</option>
        </select>
        <label class="lc-sidebar-controls__label" for="lc-sidebar-search">Search</label>
        <input id="lc-sidebar-search" class="lc-sidebar-controls__search" data-sidebar-search type="search" placeholder="Search problems" />
        <label class="lc-sidebar-controls__label" for="lc-sidebar-sort">Sort</label>
        <select id="lc-sidebar-sort" class="lc-sidebar-controls__select" data-sidebar-sort>
          <option value="number-asc">Number ↑</option>
          <option value="number-desc">Number ↓</option>
          <option value="title-asc">Title A-Z</option>
        </select>
      `;
      parentList.insertAdjacentElement('beforebegin', controls);
    }

    const filterSelect = controls.querySelector('[data-sidebar-filter]');
    const tagSelect = controls.querySelector('[data-sidebar-tag]');
    const searchInput = controls.querySelector('[data-sidebar-search]');
    const sortSelect = controls.querySelector('[data-sidebar-sort]');
    if (!filterSelect || !tagSelect || !searchInput || !sortSelect) return;

    let state = { filter: 'all', tag: 'all', search: '', sort: 'number-asc' };
    try {
      state = { ...state, ...(JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}') || {}) };
    } catch {
      state = { filter: 'all', tag: 'all', search: '', sort: 'number-asc' };
    }

    const items = problemLinks
      .map((link) => {
        const li = link.closest('li');
        if (!li) return null;
        const tags = String(link.dataset.lcTags || '')
          .split(',')
          .map((t) => t.trim())
          .filter(Boolean);
        return {
          link,
          li,
          difficulty: String(link.dataset.lcDifficulty || '').toLowerCase(),
          number: Number(link.dataset.lcProblem || 0),
          title: String(link.dataset.lcTitle || link.textContent || '').trim(),
          tags,
          tagsLower: tags.map((t) => t.toLowerCase()),
        };
      })
      .filter(Boolean);

    const uniqueTags = [...new Set(items.flatMap((item) => item.tagsLower))].sort((a, b) => a.localeCompare(b));
    uniqueTags.forEach((tag) => {
      const option = document.createElement('option');
      option.value = tag;
      option.textContent = tag
        .split(' ')
        .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
        .join(' ');
      tagSelect.appendChild(option);
    });

    if (!['all', 'easy', 'medium', 'hard'].includes(state.filter)) state.filter = 'all';
    if (!['number-asc', 'number-desc', 'title-asc'].includes(state.sort)) state.sort = 'number-asc';
    if (state.tag !== 'all' && !uniqueTags.includes(state.tag)) state.tag = 'all';

    filterSelect.value = state.filter;
    tagSelect.value = state.tag;
    searchInput.value = state.search;
    sortSelect.value = state.sort;

    function saveState() {
      state = {
        filter: filterSelect.value,
        tag: tagSelect.value,
        search: searchInput.value.trim(),
        sort: sortSelect.value,
      };
      localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    }

    function applyControls() {
      const filter = filterSelect.value;
      const tag = tagSelect.value;
      const query = searchInput.value.trim().toLowerCase();
      const sort = sortSelect.value;

      items.forEach((item) => {
        const difficultyOk = filter === 'all' || item.difficulty === filter;
        const tagOk = tag === 'all' || item.tagsLower.includes(tag);
        const searchOk = !query || item.title.toLowerCase().includes(query) || String(item.number).includes(query);
        const visible = difficultyOk && tagOk && searchOk;
        item.li.style.display = visible ? '' : 'none';
      });

      const visibleItems = items.filter((item) => item.li.style.display !== 'none');
      visibleItems.sort((a, b) => {
        if (sort === 'number-desc') return b.number - a.number;
        if (sort === 'title-asc') return a.title.toLowerCase().localeCompare(b.title.toLowerCase());
        return a.number - b.number;
      });

      visibleItems.forEach((item) => {
        parentList.appendChild(item.li);
      });

      saveState();
    }

    if (!filterSelect.dataset.lcBound) {
      filterSelect.dataset.lcBound = '1';
      filterSelect.addEventListener('change', applyControls);
    }
    if (!tagSelect.dataset.lcBound) {
      tagSelect.dataset.lcBound = '1';
      tagSelect.addEventListener('change', applyControls);
    }
    if (!searchInput.dataset.lcBound) {
      searchInput.dataset.lcBound = '1';
      searchInput.addEventListener('input', applyControls);
    }
    if (!sortSelect.dataset.lcBound) {
      sortSelect.dataset.lcBound = '1';
      sortSelect.addEventListener('change', applyControls);
    }

    applyControls();
  }

  /* ── Dev Search Guard ───────────────────────────────────── */
  function initDevSearchGuard() {
    const isLocalDev =
      window.location.hostname === 'localhost' ||
      window.location.hostname === '127.0.0.1';
    if (!isLocalDev) return;

    const searchButton = document.querySelector('button[aria-label="Search"], button[data-open-modal="starlight__search"]');
    if (searchButton && !searchButton.dataset.lcDevDisabled) {
      searchButton.dataset.lcDevDisabled = '1';
      searchButton.setAttribute('title', 'Search is available in production builds only.');
      searchButton.setAttribute('aria-disabled', 'true');
      searchButton.setAttribute('aria-label', 'Search (production builds only)');
      const shortcutHint = searchButton.querySelector('kbd');
      if (shortcutHint) shortcutHint.style.display = 'none';
      searchButton.addEventListener('click', (event) => {
        event.preventDefault();
        event.stopPropagation();
      });
    }

    if (window._lcSearchDevGuardKeyHandler) {
      document.removeEventListener('keydown', window._lcSearchDevGuardKeyHandler);
    }

    window._lcSearchDevGuardKeyHandler = (event) => {
      const isSearchChord = (event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 'k';
      if (!isSearchChord) return;
      event.preventDefault();
      event.stopPropagation();
    };

    document.addEventListener('keydown', window._lcSearchDevGuardKeyHandler, true);
  }

  /* ── Logic Player ───────────────────────────────────── */
  function initLogicPlayers() {
    document.querySelectorAll('[data-logic-player]').forEach((player) => {
      if (player.dataset.lcReady) return;
      player.dataset.lcReady = '1';

      let steps = [];
      try {
        steps = JSON.parse(player.dataset.steps || '[]');
      } catch {
        steps = [];
      }
      if (!steps.length) return;

      const itemsEl = player.querySelector('[data-items]');
      const memoryEl = player.querySelector('[data-memory]');
      const titleEl = player.querySelector('[data-step-title]');
      const noteEl = player.querySelector('[data-step-note]');
      const calloutEl = player.querySelector('[data-step-callout]');
      const formulaEl = player.querySelector('[data-formula]');
      const currentStepEl = player.querySelector('[data-current-step]');
      const progressBarEl = player.querySelector('[data-progress-bar]');
      const prevButton = player.querySelector('[data-action="prev"]');
      const nextButton = player.querySelector('[data-action="next"]');
      const playButton = player.querySelector('[data-action="play"]');
      let index = 0;
      let autoplayId = null;

      function stopAutoplay() {
        if (autoplayId) {
          window.clearInterval(autoplayId);
          autoplayId = null;
        }
        if (playButton) playButton.textContent = 'Autoplay';
      }

      function createNode(tag, className, text) {
        const el = document.createElement(tag);
        if (className) el.className = className;
        if (text !== undefined) el.textContent = text;
        return el;
      }

      function render() {
        const step = steps[index];
        currentStepEl.textContent = String(index + 1);
        progressBarEl.style.width = `${((index + 1) / steps.length) * 100}%`;
        titleEl.textContent = step.title || '';
        noteEl.textContent = step.note || '';
        calloutEl.textContent = step.callout || '';
        calloutEl.style.display = step.callout ? '' : 'none';
        formulaEl.textContent = step.formula || 'Watch the current number and its complement.';

        if (prevButton) prevButton.disabled = index === 0;
        if (nextButton) nextButton.disabled = index === steps.length - 1;

        itemsEl.innerHTML = '';
        (step.items || []).forEach((item) => {
          const wrapper = createNode('span', `lc-logic-player__item lc-logic-player__item--${item.state || 'default'}`);
          if (typeof item.index === 'number') {
            wrapper.appendChild(createNode('span', 'lc-logic-player__item-index', `i=${item.index}`));
          }
          wrapper.appendChild(createNode('span', 'lc-logic-player__item-value', item.label));
          itemsEl.appendChild(wrapper);
        });

        memoryEl.innerHTML = '';
        if ((step.memory || []).length) {
          step.memory.forEach((item) => {
            memoryEl.appendChild(createNode('span', 'lc-logic-player__memory-chip', item));
          });
        } else {
          memoryEl.appendChild(createNode('span', 'lc-logic-player__memory-empty', 'Empty'));
        }
      }

      player.querySelectorAll('[data-action]').forEach((button) => {
        button.addEventListener('click', () => {
          const action = button.getAttribute('data-action');
          if (action !== 'play') stopAutoplay();
          if (action === 'prev') index = Math.max(0, index - 1);
          if (action === 'next') index = Math.min(steps.length - 1, index + 1);
          if (action === 'reset') index = 0;
          if (action === 'play') {
            if (autoplayId) {
              stopAutoplay();
              return;
            }
            if (index === steps.length - 1) index = 0;
            playButton.textContent = 'Pause';
            autoplayId = window.setInterval(() => {
              if (index >= steps.length - 1) {
                stopAutoplay();
                render();
                return;
              }
              index += 1;
              render();
            }, 1400);
          }
          render();
        });
      });

      render();
    });
  }

  /* ── Init all ───────────────────────────────────────────── */
  function init() {
    createProgressBar();
    pageTransition();
    initSectionReveal();
    initCardStagger();
    initCardTilt();
    createBackToTop();
    initHeadingAnchors();
    initMermaidAnimation();
    initTabCrossfade();
    initCodeHover();
    initApproachNav();
    initMermaidFullscreen();
    initCopyFeedback();
    initRoadmapChecklistPersistence();
    initLangPersistence();
    initMobileAccordion();
    initSidebarCount();
    initSidebarProblemControls();
    initDevSearchGuard();
    initLogicPlayers();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init, { once: true });
  } else {
    init();
  }

  document.addEventListener('astro:page-load', init);
})();
