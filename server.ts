import { serve } from "bun";
import { join } from "path";

const PORT = 3000;
const PUBLIC_DIR = "www/dist";

console.log(`Starting server on http://localhost:${PORT}`);

serve({
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);
    let path = url.pathname;
    
    if (path === "/") path = "/index.html";
    
    const filePath = join(process.cwd(), PUBLIC_DIR, path);
    const file = Bun.file(filePath);

    if (await file.exists()) {
      return new Response(file);
    }

    return new Response("Not Found", { status: 404 });
  },
});
