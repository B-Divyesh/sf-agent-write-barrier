import { createHash } from 'node:crypto';
import { readFile, readdir, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { join, relative, resolve } from 'node:path';

async function filesWithin(directory, root = directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const files = [];
  for (const entry of entries) {
    if (entry.name === 'sw.js') continue;
    const path = join(directory, entry.name);
    if (entry.isDirectory()) files.push(...await filesWithin(path, root));
    else {
      const url = `/${relative(root, path).replaceAll('\\', '/')}`
        .replace(/\/index\.html$/, '/');
      files.push(url);
    }
  }
  return files;
}

/**
 * Emit the offline worker for one static deployment.
 *
 * The cache identifier is derived from both each URL and its bytes. A change
 * to HTML with unchanged asset URLs therefore still changes sw.js and prompts
 * existing browsers to install a fresh worker.
 */
export async function buildServiceWorker(root = resolve('dist/site')) {
  const assets = (await filesWithin(root)).sort();
  const manifest = await Promise.all(assets.map(async (url) => ({
    url,
    hash: createHash('sha256').update(await readFile(url.endsWith('/')
      ? join(root, url.slice(1), 'index.html')
      : join(root, url.slice(1)))).digest('hex')
  })));
  const cacheVersion = createHash('sha256')
    .update(JSON.stringify(manifest))
    .digest('hex')
    .slice(0, 20);
  const source = `const CACHE = 'awb-site-${cacheVersion}';
const SHELL = ${JSON.stringify(assets)};
self.addEventListener('install', (event) => event.waitUntil(caches.open(CACHE).then((cache) => cache.addAll(SHELL)).then(() => self.skipWaiting())));
self.addEventListener('activate', (event) => event.waitUntil(caches.keys().then((keys) => Promise.all(keys.filter((key) => key !== CACHE).map((key) => caches.delete(key)))).then(() => self.clients.claim())));
self.addEventListener('fetch', (event) => {
  if (event.request.method !== 'GET' || new URL(event.request.url).origin !== self.location.origin) return;
  if (event.request.mode === 'navigate') {
    event.respondWith(fetch(event.request).then((response) => {
      const copy = response.clone();
      caches.open(CACHE).then((cache) => cache.put(event.request, copy));
      return response;
    }).catch(() => caches.match(event.request).then((cached) => cached || caches.match('/'))));
    return;
  }
  event.respondWith(caches.match(event.request).then((cached) => cached || fetch(event.request).then((response) => {
    const copy = response.clone();
    caches.open(CACHE).then((cache) => cache.put(event.request, copy));
    return response;
  })));
});
`;
await writeFile(join(root, 'sw.js'), source);
console.log(`Precached ${assets.length} site files`);
  return { assets, cacheVersion, source };
}

if (process.argv[1] && resolve(fileURLToPath(import.meta.url)) === resolve(process.argv[1])) {
  await buildServiceWorker();
}
