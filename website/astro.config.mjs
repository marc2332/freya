import { defineConfig } from 'astro/config';
import { satteri, satteriHeadingIdsPlugin } from '@astrojs/markdown-satteri';
import tailwind from "@astrojs/tailwind";

const headingAnchors = {
  name: "heading-anchors",
  element: {
    filter: ["h1", "h2", "h3", "h4", "h5", "h6"],
    visit(node, ctx) {
      ctx.appendChild(node, {
        type: "raw",
        value: `<a href="#${node.properties.id}" aria-hidden="true" tabindex="-1"><span class="icon icon-link"></span></a>`,
      });
    },
  },
};

// https://astro.build/config
export default defineConfig({
  site: "https://freyaui.dev",
  integrations: [tailwind()],
  server: {
    allowedHosts: [".ngrok-free.app"],
  },
  markdown: {
    processor: satteri({
      hastPlugins: [satteriHeadingIdsPlugin(), headingAnchors],
    }),
  },
});
