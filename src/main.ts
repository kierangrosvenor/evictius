import { createApp } from "vue";
import App from "./App.vue";

import {ClickOutsideDirective} from './directives/ClickOutsideDirective'

// Turn on .dark when the OS is in dark mode, and follow it if it changes.
const darkQuery = window.matchMedia("(prefers-color-scheme: dark)");
const applyTheme = () => document.documentElement.classList.toggle("dark", darkQuery.matches);
applyTheme();
darkQuery.addEventListener("change", applyTheme);

const app = createApp(App)
app.directive('click-outside', ClickOutsideDirective);


app.mount("#app");
