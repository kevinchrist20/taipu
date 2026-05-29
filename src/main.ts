import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { createPinia } from "pinia";
import '@unocss/reset/tailwind-compat.css';
import './styles/theme.css';
import 'virtual:uno.css';

const pinia = createPinia();

createApp(App)
    .use(pinia)
    .use(router)
    .mount("#app");
