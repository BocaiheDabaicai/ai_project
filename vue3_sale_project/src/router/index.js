import { createRouter, createWebHashHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('../views/Login.vue'),
    meta: { requiresAuth: false }
  },
  {
    path: '/',
    component: () => import('../layouts/MainLayout.vue'),
    meta: { requiresAuth: true },
    redirect: '/purchase',
    children: [
      {
        path: 'purchase',
        name: 'PurchaseOrder',
        component: () => import('../views/purchase/PurchaseOrder.vue'),
        meta: { title: '采购订单管理' }
      },
      {
        path: 'supplier',
        name: 'SupplierManage',
        component: () => import('../views/supplier/SupplierManage.vue'),
        meta: { title: '供应商管理' }
      },
      {
        path: 'inventory',
        name: 'InventoryManage',
        component: () => import('../views/inventory/InventoryManage.vue'),
        meta: { title: '库存管理' }
      }
    ]
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

router.beforeEach((to, from, next) => {
  const authStore = useAuthStore()
  if (to.meta.requiresAuth !== false && !authStore.isLoggedIn) {
    next({ name: 'Login', query: { redirect: to.fullPath } })
  } else if (to.name === 'Login' && authStore.isLoggedIn) {
    next({ path: '/' })
  } else {
    next()
  }
})

export default router
