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
    redirect: '/procurement/dashboard',
    children: [
      // ===== 采购人员模块 =====
      {
        path: 'procurement/dashboard',
        name: 'ProcurementDashboard',
        component: () => import('../views/procurement/Dashboard.vue'),
        meta: { title: '采购工作台' }
      },
      {
        path: 'procurement/requisition',
        name: 'ProcurementRequisition',
        component: () => import('../views/procurement/RequisitionList.vue'),
        meta: { title: '请购单管理' }
      },
      {
        path: 'procurement/order-list',
        name: 'ProcurementOrderList',
        component: () => import('../views/procurement/OrderList.vue'),
        meta: { title: '采购订单管理' }
      },
      {
        path: 'procurement/order-detail/:id',
        name: 'ProcurementOrderDetail',
        component: () => import('../views/procurement/OrderDetail.vue'),
        meta: { title: '采购订单详情' }
      },
      {
        path: 'procurement/settings',
        name: 'ProcurementSettings',
        component: () => import('../views/procurement/Settings.vue'),
        meta: { title: '采购配置' }
      },
      // ===== 原始页面（保留） =====
      {
        path: 'purchase',
        name: 'PurchaseOrder',
        component: () => import('../views/purchase/PurchaseOrder.vue'),
        meta: { title: '采购订单管理(旧)' }
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
