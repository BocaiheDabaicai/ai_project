<script setup>
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useThemeStore } from '../stores/theme'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const themeStore = useThemeStore()

const menuItems = [
  { path: '/procurement/dashboard', name: 'ProcurementDashboard', label: '采购工作台', icon: 'Monitor' },
  { path: '/procurement/requisition', name: 'ProcurementRequisition', label: '请购单管理', icon: 'Tickets' },
  { path: '/procurement/order-list', name: 'ProcurementOrderList', label: '采购订单管理', icon: 'Document' },
  { path: '/procurement/settings', name: 'ProcurementSettings', label: '采购配置', icon: 'Setting' },
  { path: '/supplier', name: 'SupplierManage', label: '供应商管理', icon: 'OfficeBuilding' },
  { path: '/inventory', name: 'InventoryManage', label: '库存管理', icon: 'Box' },
]

const activeMenu = computed(() => route.path)

function navigate(item) {
  router.push(item.path)
}

function handleLogout() {
  authStore.logout()
  router.push('/login')
}

function switchTheme(key) {
  themeStore.applyTheme(key)
}
</script>

<template>
  <div class="layout-container">
    <!-- 侧边栏 -->
    <aside class="layout-sidebar">
      <div class="sidebar-header">
        <el-icon :size="22" color="var(--color-accent)"><ShoppingCartFull /></el-icon>
        <span class="sidebar-title">采购运营平台</span>
      </div>
      <nav class="sidebar-nav">
        <div
          v-for="item in menuItems"
          :key="item.path"
          class="menu-item"
          :class="{ active: activeMenu === item.path }"
          @click="navigate(item)"
        >
          <el-icon :size="18"><component :is="item.icon" /></el-icon>
          <span class="menu-label">{{ item.label }}</span>
        </div>
      </nav>
      <div class="sidebar-footer">
        <span class="footer-ver">v1.0.0</span>
      </div>
    </aside>

    <!-- 右侧主体 -->
    <div class="layout-main">
      <header class="layout-header">
        <div class="header-left">
          <span class="header-page-name">
            {{ menuItems.find(m => m.path === activeMenu)?.label || '' }}
          </span>
        </div>
        <div class="header-right">
          <!-- 主题切换 -->
          <el-dropdown trigger="click" @command="switchTheme">
            <span class="theme-trigger">
              <el-icon :size="18"><Brush /></el-icon>
              <span class="theme-label">主题</span>
            </span>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item
                  v-for="t in themeStore.themes"
                  :key="t.key"
                  :command="t.key"
                  :class="{ 'is-active': themeStore.current === t.key }"
                >
                  <span class="theme-dot" :style="{ background: t.color }"></span>
                  <span>{{ t.label }}</span>
                  <el-icon v-if="themeStore.current === t.key" :size="14" style="margin-left:auto"><Check /></el-icon>
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>

          <!-- 用户信息 -->
          <el-dropdown trigger="click">
            <span class="user-trigger">
              <el-avatar :size="28" icon="UserFilled" />
              <span class="user-name">{{ authStore.user?.username }}</span>
            </span>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item disabled>
                  <span style="color:var(--text-secondary)">角色: {{ authStore.user?.role }}</span>
                </el-dropdown-item>
                <el-dropdown-item divided @click="handleLogout">
                  <el-icon :size="14"><SwitchButton /></el-icon>
                  <span>退出登录</span>
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </header>
      <main class="layout-content">
        <router-view />
      </main>
    </div>
  </div>
</template>

<style scoped>
.layout-container {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background: var(--bg-page);
}

/* ===== 侧边栏 ===== */
.layout-sidebar {
  width: 230px;
  background: var(--bg-sidebar);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
  border-right: 1px solid var(--border-color);
  transition: background .3s;
}
.sidebar-header {
  height: 60px;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 0 18px;
  border-bottom: 1px solid var(--border-color);
}
.sidebar-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: .5px;
}
.sidebar-nav {
  flex: 1;
  padding: 8px 0;
  overflow-y: auto;
}
.menu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 11px 20px;
  margin: 2px 10px;
  cursor: pointer;
  font-size: 14px;
  border-radius: var(--radius-sm);
  color: var(--color-sidebar-text);
  transition: all .2s ease;
}
.menu-item:hover {
  background: var(--bg-sidebar-hover);
}
.menu-item.active {
  background: var(--bg-sidebar-active);
  color: var(--color-sidebar-text-active);
  font-weight: 500;
  box-shadow: var(--shadow-sm);
}
.menu-label {
  white-space: nowrap;
}
.sidebar-footer {
  padding: 12px 20px;
  border-top: 1px solid var(--border-color);
}
.footer-ver {
  font-size: 11px;
  color: var(--text-muted);
}

/* ===== 主体区 ===== */
.layout-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* ===== 顶栏 ===== */
.layout-header {
  height: 52px;
  background: var(--bg-header);
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 24px;
  flex-shrink: 0;
  box-shadow: var(--shadow-sm);
}
.header-left {
  display: flex;
  align-items: center;
}
.header-page-name {
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
}
.header-right {
  display: flex;
  align-items: center;
  gap: 20px;
}
.theme-trigger {
  display: flex;
  align-items: center;
  gap: 5px;
  cursor: pointer;
  color: var(--text-secondary);
  font-size: 13px;
  transition: color .2s;
  user-select: none;
}
.theme-trigger:hover {
  color: var(--color-accent);
}
.theme-dot {
  display: inline-block;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  margin-right: 8px;
  box-shadow: 0 0 4px rgba(0,0,0,.15);
}
.user-trigger {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}
.user-name {
  font-size: 13px;
  color: var(--text-primary);
}

/* ===== 内容区 ===== */
.layout-content {
  flex: 1;
  padding: 20px 24px;
  overflow-y: auto;
}
</style>
