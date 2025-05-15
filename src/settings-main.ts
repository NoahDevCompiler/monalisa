import { createApp } from 'vue';
import App from './Views/SettingsWindow.vue';
import router from './router';
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import './assets/tailwind.css';
import store from './store.ts'

createApp(App)
  .use(router)
  .use(ElementPlus)
  .provide('store', store)
  .mount('#settings-app');