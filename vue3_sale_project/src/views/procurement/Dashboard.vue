<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const stats = ref([
  { label: '待处理请购单', value: 8, color: '#e6a23c', icon: 'Tickets' },
  { label: '进行中订单', value: 12, color: '#3498db', icon: 'Document' },
  { label: '今日已处理', value: 5, color: '#67c23a', icon: 'CircleCheck' },
  { label: '待比价物料', value: 3, color: '#f56c6c', icon: 'Warning' },
])

const activities = ref([
  { id: 1, text: '请购单 REQ20260510008 已推送至采购部', time: '10分钟前', type: 'warning' },
  { id: 2, text: '采购订单 PO20260509007 供应商已发货', time: '1小时前', type: 'success' },
  { id: 3, text: '请购单 REQ20260509006 比价完成，待分配数量', time: '2小时前', type: '' },
  { id: 4, text: '采购订单 PO20260508005 已签收', time: '5小时前', type: 'success' },
  { id: 5, text: '物料缺价提醒：不锈钢板材 304/2mm 缺少供应商报价', time: '6小时前', type: 'danger' },
])

function go(path) {
  router.push(path)
}
</script>

<template>
  <div class="dashboard">
    <h2 class="page-title">
      <el-icon :size="20" color="var(--color-accent)"><Monitor /></el-icon>
      采购工作台
    </h2>

    <!-- 统计卡片 -->
    <div class="stats-row">
      <div
        v-for="s in stats"
        :key="s.label"
        class="stat-card"
        @click="s.label === '待处理请购单' ? go('/procurement/requisition') : s.label === '进行中订单' ? go('/procurement/order-list') : undefined"
        :style="{ cursor: s.label === '待处理请购单' || s.label === '进行中订单' ? 'pointer' : 'default' }"
      >
        <div class="stat-value" :style="{ color: s.color }">{{ s.value }}</div>
        <div class="stat-label">{{ s.label }}</div>
      </div>
    </div>

    <!-- 快捷入口 -->
    <div class="section">
      <h3 class="section-title">快捷操作</h3>
      <div class="quick-actions">
        <div class="action-card" @click="go('/procurement/requisition')">
          <el-icon :size="32" color="#e6a23c"><Tickets /></el-icon>
          <span>请购单管理</span>
          <small>查看并处理请购单</small>
        </div>
        <div class="action-card" @click="go('/procurement/order-list')">
          <el-icon :size="32" color="#3498db"><Document /></el-icon>
          <span>采购订单</span>
          <small>管理所有采购订单</small>
        </div>
        <div class="action-card" @click="go('/procurement/settings')">
          <el-icon :size="32" color="#8e44ad"><Setting /></el-icon>
          <span>采购配置</span>
          <small>物料类型与标包设置</small>
        </div>
      </div>
    </div>

    <!-- 动态 + 待办 -->
    <div class="bottom-row">
      <div class="section" style="flex:2">
        <h3 class="section-title">最近动态</h3>
        <div class="activity-list">
          <div
            v-for="act in activities"
            :key="act.id"
            class="activity-item"
          >
            <span class="activity-dot" :style="{ background: `var(--el-color-${act.type || 'info'})` }"></span>
            <span class="activity-text">{{ act.text }}</span>
            <span class="activity-time">{{ act.time }}</span>
          </div>
        </div>
      </div>
      <div class="section" style="flex:1">
        <h3 class="section-title">待办提醒</h3>
        <div class="todo-list">
          <div class="todo-item"><el-icon color="#e6a23c"><Warning /></el-icon> 3 个请购单等待比价</div>
          <div class="todo-item"><el-icon color="#e6a23c"><Warning /></el-icon> 2 个订单待分配数量</div>
          <div class="todo-item"><el-icon color="#3498db"><InfoFilled /></el-icon> 5 个物料需要补价</div>
          <div class="todo-item"><el-icon color="#67c23a"><CircleCheck /></el-icon> 今日已处理 5 单</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dashboard {
  max-width: 1200px;
}
.section {
  background: var(--bg-card);
  border-radius: var(--radius);
  padding: 20px;
  margin-bottom: 16px;
}
.section-title {
  font-size: 15px;
  font-weight: 600;
  margin-bottom: 16px;
  color: var(--text-primary);
}
.quick-actions {
  display: flex;
  gap: 16px;
}
.action-card {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 28px 20px;
  background: var(--bg-page);
  border-radius: var(--radius);
  cursor: pointer;
  transition: all .2s;
  text-align: center;
}
.action-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}
.action-card span {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}
.action-card small {
  font-size: 12px;
  color: var(--text-secondary);
}
.bottom-row {
  display: flex;
  gap: 16px;
}
.activity-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 0;
  border-bottom: 1px solid var(--border-light);
}
.activity-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}
.activity-text {
  flex: 1;
  font-size: 13px;
  color: var(--text-primary);
}
.activity-time {
  font-size: 11px;
  color: var(--text-muted);
  flex-shrink: 0;
}
.todo-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.todo-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: var(--text-primary);
}
</style>
