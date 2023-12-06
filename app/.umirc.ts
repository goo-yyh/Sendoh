import { defineConfig } from '@umijs/max';

export default defineConfig({
  antd: {},
  access: {},
  model: {},
  initialState: {},
  request: {},
  layout: {
    title: '@umijs/max',
  },
  routes: [
    {
      path: '/',
      redirect: '/home',
    },
    {
      name: '模块差异分析',
      path: '/home',
      component: './Home',
    },
    {
      name: '代码差异分析',
      path: '/access',
      component: './Access',
    }
  ],
  npmClient: 'pnpm',
});

