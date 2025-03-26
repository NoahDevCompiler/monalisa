import { createApp } from 'vue';
import App from "./App.vue";
import ElementPlus from 'element-plus'
import { MotionPlugin } from '@vueuse/motion'
import 'element-plus/dist/index.css'
import './assets/tailwind.css'
import router from './router';

const app = createApp(App)

app.use(router)
app.use(ElementPlus)
app.use(MotionPlugin)
app.mount('#app')
