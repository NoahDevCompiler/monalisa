import { createApp } from 'vue';
import App from "./App.vue";
import ElementPlus from 'element-plus'
import { MotionPlugin } from '@vueuse/motion'
import 'element-plus/dist/index.css'
import './assets/tailwind.css'
import store from "./store.ts"
import router from './router';

const app = createApp(App)

app.use(router)
app.use(ElementPlus)
app.use(MotionPlugin)
app.provide("store", store)
app.mount('#app')
