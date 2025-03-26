import { createApp } from 'vue';
import App from './Elements/SettingsWindow.vue';
import router from './router';
import './assets/tailwind.css';

createApp(App)
  .use(router)
  .mount('#settings-app');