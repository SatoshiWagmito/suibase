//import * as path from "path";
import { defineUserConfig } from "vuepress";
import theme from "./theme.js";
//import { searchProPlugin } from "vuepress-plugin-search-pro";
import { docsearchPlugin } from "@vuepress/plugin-docsearch";
//import { registerComponentsPlugin } from "@vuepress/plugin-register-components";
//import { redirectPlugin } from "vuepress-plugin-redirect";
import { googleAnalyticsPlugin } from "@vuepress/plugin-google-analytics";

export default defineUserConfig({
  base: "/",

  locales: {
    "/": {
      lang: "en-US",
      title: "suibase.io",
      description:
        "Sui Network Open-Source Development Tools and Community Cookbook",
    },
  },

  theme,

  plugins: [
    /*
    registerComponentsPlugin({
      componentsDir: path.resolve(__dirname, "./components"),
    }),*/
    docsearchPlugin({
      // your options
      // appId, apiKey and indexName are required
      appId: "VN5D5IVTPC",
      apiKey: "7c6732e9f43a129ee2396d1c459db319",
      indexName: "sui-base",
    }),
    googleAnalyticsPlugin({
      id: "G-JVE9L5ZDYZ",
    }),
  ],

  // Enable it with pwa
  // shouldPrefetch: false,
});
