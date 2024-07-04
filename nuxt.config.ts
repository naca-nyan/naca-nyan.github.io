// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  app: {
    head: {
      title: "naca-nyan",
      titleTemplate: "%s - naca-nyan.github.io",
    },
  },
  css: ["sakura.css/css/sakura-earthly.css", "/css/style.css"],
});
