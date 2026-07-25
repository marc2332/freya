import { defineMiddleware } from "astro:middleware";

const hostRedirects: Record<string, string> = {
  "book.freyaui.dev": "https://docs.rs/freya",
  "docs.freyaui.dev": "https://docs.rs/freya",
};

export const onRequest = defineMiddleware((context, next) => {
  const host = context.request.headers.get("host");
  const destination = host && hostRedirects[host];
  if (destination) {
    return context.redirect(destination, 301);
  }
  return next();
});
