self.addEventListener('install', (e) => {
  e.waitUntil(
    caches.use('game00-cache-v1').then((cache) => {
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
    caches.match(e.request).then((response) => {
      return response || fetch(e.request);
    })
  );
});