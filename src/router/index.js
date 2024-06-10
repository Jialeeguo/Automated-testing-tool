import { createRouter, createWebHistory } from 'vue-router';
import RecordPage from '../components/RecordPage.vue';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: {
      template: '<div><h1>Welcome to the Home Page</h1></div>',
    }
  },
  {
    path: '/second',
    name: 'Second',
    component: RecordPage
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
