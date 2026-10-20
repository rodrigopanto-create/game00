const CACHE_NAME = 'game00-cache-v2';

self.addEventListener('install', (e) => {
  e.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
      return cache.addAll([
        './index.html',
        './mq_js_bundle.js',
        './game00.wasm'
      ]);
    })
  );
});

self.addEventListener('fetch', (e) => {
  e.respondWith(
    caches.open(CACHE_NAME).then((cache) => cache.match(e.request)).then((response) => {
      return response || fetch(e.request);
    })
  );
});