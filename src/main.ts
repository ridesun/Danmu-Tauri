import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import 'vfonts/Lato.css'
import main from './components/Main.vue';
import danmu from './components/Danmu.vue';
import { createRouter, createWebHashHistory } from 'vue-router'

const routes=[{path:'/',component:main},{path:'/danmu/:id',component:danmu}];

const router = createRouter({
    history: createWebHashHistory(),
    routes, 
  })

createApp(App)
    .use(router)
    .mount("#app");
