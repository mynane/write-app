import { createWebHashHistory, createRouter } from 'vue-router'
import Home from './pages/home.vue'
import Regulex from './pages/regulex.vue'
import Setting from './pages/setting.vue'

const routes = [
  { path: '/', component: Home },
  // { path: '/regulex', component: Regulex },
  { path: '/setting', component: Setting },
]

export default createRouter({
  // 内部提供了 history 模式的实现。为了简单起见，我们在这里使用 hash 模式。
  history: createWebHashHistory(),
  routes, // `routes: routes` 的缩写
})
