import { createApp } from "vue";
import 'virtual:uno.css';
import './assets/styles.css';
import App from "./App.vue";
import router from "./router";
import { createPinia } from "pinia";


const pinia = createPinia();

createApp(App)
    .use(pinia)
    .use(router)
    .mount("#app");
