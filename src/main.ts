import { mount } from "svelte";
import "./app.css";
import { setupI18n, getInitialLang } from "$lib/i18n";
import App from "./App.svelte";

setupI18n(getInitialLang());

const app = mount(App, { target: document.getElementById("app")! });

export default app;
