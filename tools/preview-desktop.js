// Development-only HTTP preview. It exposes only the three compiled UI assets on loopback.
import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
const files = new Map([
  ['/', ['index.html', 'text/html; charset=utf-8']],
  ['/main.js', ['main.js', 'text/javascript; charset=utf-8']],
  ['/style.css', ['style.css', 'text/css; charset=utf-8']],
]);
const server = createServer(async (request, response) => {
  const file = files.get(request.url?.split('?')[0]);
  if (!file) { response.writeHead(404).end(); return; }
  try {
    const bytes = await readFile(new URL('../apps/desktop/dist/' + file[0], import.meta.url));
    response.writeHead(200, { 'Content-Type': file[1], 'Cache-Control': 'no-store' }).end(bytes);
  } catch { response.writeHead(503).end('Execute npm run build:ui primeiro.'); }
});
server.listen(4173, '127.0.0.1', () => console.log('Desktop UI preview: http://127.0.0.1:4173'));
