import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { createPinia } from "pinia";
import { OhVueIcon, addIcons } from "oh-vue-icons";
import { FcLock, FcOk } from "oh-vue-icons/icons";
import 'virtual:uno.css';
import '@unocss/reset/tailwind-compat.css'

const pinia = createPinia();

addIcons(FcLock, FcOk);

createApp(App)
    .use(pinia)
    .use(router)
    .component("v-icon", OhVueIcon)
    .mount("#app");
