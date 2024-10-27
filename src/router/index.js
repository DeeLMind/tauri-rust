import { createRouter, createWebHashHistory } from 'vue-router';


const routes = [
{
    path: '/',
    component: () => import('../Main.vue'),
},
  {
    path: '/home',
    component: () => import('../Home.vue'),
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

export default router;