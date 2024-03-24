import { defineConfig } from 'astro/config';
import { rehypeAccessibleEmojis } from 'rehype-accessible-emojis';
import rehypeAutoLinks from 'rehype-autolink-headings';
import rehypeSlug from 'rehype-slug'
import deno from "@astrojs/deno";
import tailwind from "@astrojs/tailwind";

// https://astro.build/config
export default defineConfig({
  site: "https://freyaui.dev",
  integrations: [tailwind()],
  output: "server",
  adapter: deno(),
  markdown: {
    rehypePlugins: [rehypeAccessibleEmojis, rehypeSlug, () => rehypeAutoLinks({
      behavior: "append",
    })]
  }
});