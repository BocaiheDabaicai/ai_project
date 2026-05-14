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
    redirect: '/procurement',
    children: [
      // ===== 采购模块总入口 =====
      {
        path: 'procurement',
        name: 'ProcurementDashboard',
        component: () => import('../views/procurement/Dashboard.vue'),
        meta: { title: '采购管理' }
      },
      // ===== 纸箱采购 =====
      {
        path: 'procurement/carton/requisition',
        name: 'CartonRequisition',
        component: () => import('../views/procurement/carton/RequisitionList.vue'),
        meta: { title: '纸箱请购单' }
      },
      {
        path: 'procurement/carton/create-order/:reqId',
        name: 'CartonCreateOrder',
        component: () => import('../views/procurement/carton/CreateOrder.vue'),
        meta: { title: '创建采购订单' }
      },
      {
        path: 'procurement/carton/order-list',
        name: 'CartonOrderList',
        component: () => import('../views/procurement/carton/OrderList.vue'),
        meta: { title: '纸箱采购订单' }
      },
      {
        path: 'procurement/carton/order-detail/:id',
        name: 'CartonOrderDetail',
        component: () => import('../views/procurement/carton/OrderDetail.vue'),
        meta: { title: '订单详情' }
      },
      // ===== 其他模块占位 =====
      {
        path: 'procurement/office',
        name: 'OfficeSupplies',
        component: () => import('../views/procurement/OfficeSupplies.vue'),
        meta: { title: '办公用品采购' }
      },
      {
        path: 'procurement/ranch',
        name: 'RanchMaterials',
        component: () => import('../views/procurement/RanchMaterials.vue'),
        meta: { title: '牧场物料采购' }
      },
      {
        path: 'procurement/cold-chain',
        name: 'ColdChain',
        component: () => import('../views/procurement/ColdChain.vue'),
        meta: { title: '低温包装采购' }
      },
      {
        path: 'procurement/summary',
        name: 'ProcurementSummary',
        component: () => import('../views/procurement/Summary.vue'),
        meta: { title: '采购汇总' }
      },
      // ===== 保留 =====
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
