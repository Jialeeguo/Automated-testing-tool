import { createRouter, createWebHashHistory} from 'vue-router';
import transcribe from './components/transcribe.vue';
import ide from './components/ide.vue';

const routes = [
  {
    path: '/',
    component: transcribe,
  },
  {
    path: '/ide',
    component: ide, 
  },
  // 其他路由规则
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
