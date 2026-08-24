import { serveDir } from "jsr:@std/http/file-server";

const hostRedirects: Record<string, string> = {
  "book.freyaui.dev": "https://docs.rs/freya",
  "docs.freyaui.dev": "https://docs.rs/freya",
};

Deno.serve((request) => {
  const url = new URL(request.url);
  const destination = hostRedirects[url.hostname];
  if (destination) {
    return Response.redirect(destination, 301);
  }
  return serveDir(request, { fsRoot: "." });
});
