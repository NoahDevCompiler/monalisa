import { createRouter, createWebHistory } from 'vue-router';
import Main from '../App.vue'
import Settings from '../Views/SettingsWindow.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Main },
    { path: '/settings', component: Settings },
  ],
});

export default router;