import { createApp } from "vue";
import App from "./App.vue";

// Turn on .dark when the OS is in dark mode, and follow it if it changes.
const darkQuery = window.matchMedia("(prefers-color-scheme: dark)");
const applyTheme = () => document.documentElement.classList.toggle("dark", darkQuery.matches);
applyTheme();
darkQuery.addEventListener("change", applyTheme);

createApp(App).mount("#app");
