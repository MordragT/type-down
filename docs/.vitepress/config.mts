import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "TypeDown",
  description: "TypeDown Documentation",
  srcDir: "./src",
  base: "/type-down/",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Guide", link: "/guide/about" },
      { text: "Reference", link: "/reference/cli" },
    ],

    sidebar: {
      "/guide/": {
        base: "/guide/",
        items: [
          {
            text: "Introduction",
            collapsed: false,
            items: [
              { text: "What is TypeDown?", link: "about" },
              { text: "Getting Started", link: "quickstart" },
            ],
          },
          {
            text: "Writing",
            collapsed: false,
            items: [],
          },
          {
            text: "Customization",
            collapsed: false,
            items: [],
          },
        ],
      },
      "reference/": {
        base: "/reference/",
        items: [
          {
            text: "Reference",
            items: [
              { text: "CLI", link: "cli" },
              {
                text: "Blocks",
                items: [
                  { text: "Heading", link: "heading" },
                  { text: "List", link: "list" },
                  { text: "Enum", link: "enum" },
                  { text: "Terms", link: "terms" },
                  { text: "Table", link: "table" },
                  { text: "Raw", link: "raw" },
                  { text: "Paragraph", link: "paragraph" },
                ],
              },
              { text: "Inline", link: "inline" },
              { text: "Code", link: "code" },
            ],
          },
        ],
      },
    },

    socialLinks: [
      { icon: "github", link: "https://github.com/mordragt/type-down" },
    ],

    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © 2025-present Thomas Wehmöller",
    },
  },
});
