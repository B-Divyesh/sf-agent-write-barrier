import '@fontsource-variable/manrope/wght.css';
import './styles.css';

const DEMO_PREFIX = 'demo:awb:';
const KNOWN_ROUTES = new Set(['/', '/demo/', '/privacy/', '/terms/', '/404.html']);

function normalizedPath(pathname = window.location.pathname): string {
  if (pathname === '/demo' || pathname === '/privacy' || pathname === '/terms') return `${pathname}/`;
  return pathname;
}

function updateConnection(): void {
  const bar = document.querySelector<HTMLElement>('#offline-bar');
  if (bar) bar.hidden = navigator.onLine;
}

function clearDemoStorage(): void {
  for (let index = localStorage.length - 1; index >= 0; index -= 1) {
    const key = localStorage.key(index);
    if (key?.startsWith(DEMO_PREFIX)) localStorage.removeItem(key);
  }
}

function bindPageActions(): void {
  document.querySelectorAll<HTMLButtonElement>('[data-copy]').forEach((button) => {
    button.onclick = async () => {
      const target = document.getElementById(button.dataset.copy ?? '');
      if (!target) return;
      try {
        await navigator.clipboard.writeText(target.textContent ?? '');
        const original = button.textContent;
        button.textContent = 'Copied';
        window.setTimeout(() => { button.textContent = original; }, 1600);
      } catch {
        button.textContent = 'Select the command';
        const range = document.createRange();
        range.selectNodeContents(target);
        const selection = window.getSelection();
        selection?.removeAllRanges();
        selection?.addRange(range);
      }
    };
  });

  if (normalizedPath() === '/demo/') {
    localStorage.setItem(`${DEMO_PREFIX}session`, 'bundled-sample');
    const reset = () => {
      clearDemoStorage();
      localStorage.setItem(`${DEMO_PREFIX}session`, 'bundled-sample');
      const status = document.querySelector<HTMLElement>('#demo-reset-status');
      if (status) status.textContent = 'Demo reset. The original sample receipt is shown.';
      document.querySelector<HTMLElement>('.recording')?.scrollTo({ top: 0, behavior: 'instant' });
    };
    document.querySelector<HTMLButtonElement>('#reset-demo')?.addEventListener('click', reset);
    document.querySelector<HTMLButtonElement>('#reset-demo-main')?.addEventListener('click', reset);
    for (const id of ['leave-demo', 'leave-demo-main']) {
      document.getElementById(id)?.addEventListener('click', clearDemoStorage);
    }
  }

  document.querySelector<HTMLButtonElement>('#clear-site-data')?.addEventListener('click', async () => {
    clearDemoStorage();
    if ('caches' in window) {
      await Promise.all((await caches.keys()).map((key) => caches.delete(key)));
    }
    if ('serviceWorker' in navigator) {
      const registrations = await navigator.serviceWorker.getRegistrations();
      await Promise.all(registrations.map((registration) => registration.unregister()));
    }
    const status = document.querySelector<HTMLElement>('#clear-status');
    if (status) status.textContent = 'Demo and offline data cleared.';
  });
}

function syncNavigation(path: string): void {
  document.querySelectorAll<HTMLAnchorElement>('.site-header [aria-current="page"], footer [aria-current="page"]').forEach((link) => link.removeAttribute('aria-current'));
  document.querySelectorAll<HTMLAnchorElement>('.site-header a, footer a').forEach((link) => {
    if (new URL(link.href, window.location.href).pathname === path) link.setAttribute('aria-current', 'page');
  });
}

function copyMetadata(source: Document): void {
  document.title = source.title;
  const selectors = ['meta[name="description"]', 'meta[name="robots"]', 'link[rel="canonical"]', 'meta[property^="og:"]', 'meta[name^="twitter:"]'];
  for (const selector of selectors) {
    document.head.querySelectorAll(selector).forEach((node) => node.remove());
    source.head.querySelectorAll(selector).forEach((node) => document.head.append(node.cloneNode(true)));
  }
}

async function renderRoute(path: string, focus = true): Promise<void> {
  const target = KNOWN_ROUTES.has(path) ? path : '/404.html';
  const response = await fetch(target, { headers: { 'X-AWB-Route': '1' } });
  if (!response.ok && target !== '/404.html') throw new Error(`Route failed: ${response.status}`);
  const parsed = new DOMParser().parseFromString(await response.text(), 'text/html');
  const nextMain = parsed.querySelector<HTMLElement>('main');
  const currentMain = document.querySelector<HTMLElement>('main');
  if (!nextMain || !currentMain) return;
  currentMain.replaceWith(nextMain);
  copyMetadata(parsed);
  syncNavigation(path);
  bindPageActions();
  window.scrollTo({ top: 0, behavior: 'instant' });
  const heading = nextMain.querySelector<HTMLElement>('h1');
  const status = document.querySelector<HTMLElement>('#route-status');
  if (status) status.textContent = heading?.textContent ? `${heading.textContent} page loaded` : 'Page loaded';
  if (focus) heading?.focus({ preventScroll: true });
}

document.addEventListener('click', (event) => {
  if (!(event.target instanceof Element)) return;
  const link = event.target.closest<HTMLAnchorElement>('a[data-route]');
  if (!link || event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
  const url = new URL(link.href, window.location.href);
  if (url.origin !== window.location.origin) return;
  event.preventDefault();
  history.pushState({ route: url.pathname }, '', url.pathname);
  void renderRoute(normalizedPath(url.pathname));
});

window.addEventListener('popstate', () => { void renderRoute(normalizedPath()); });
window.addEventListener('online', updateConnection);
window.addEventListener('offline', updateConnection);

async function start(): Promise<void> {
  let path = normalizedPath();
  if (new URLSearchParams(window.location.search).get('demo') === '1') {
    path = '/demo/';
    history.replaceState({ route: path }, '', path);
    await renderRoute(path, false);
  } else if (!KNOWN_ROUTES.has(path)) {
    await renderRoute('/404.html', false);
  }
  syncNavigation(path);
  bindPageActions();
  updateConnection();
  if ('serviceWorker' in navigator && import.meta.env.PROD) {
    void navigator.serviceWorker.register('/sw.js', { updateViaCache: 'none' });
  }
}

void start();
