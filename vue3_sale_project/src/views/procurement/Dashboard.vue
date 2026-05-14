<template>
  <div class="page-container">
    <div class="dashboard">
      <div class="page-header">
        <h1 class="page-title">采购管理</h1>
        <p class="page-subtitle">采购模块工作台，请选择以下功能模块进入</p>
      </div>

      <!-- Stats Row -->
      <el-row :gutter="16" class="stats-row">
        <el-col :span="6" v-for="stat in stats" :key="stat.label">
          <el-card
            shadow="hover"
            class="stat-card"
            :style="{ borderLeft: `4px solid ${stat.color}` }"
          >
            <div class="stat-content">
              <div class="stat-value" :style="{ color: stat.color }">
                {{ stat.value }}
              </div>
              <div class="stat-label">{{ stat.label }}</div>
            </div>
          </el-card>
        </el-col>
      </el-row>

      <!-- Module Cards -->
      <el-row :gutter="20" class="modules-row">
        <el-col
          :xs="24"
          :sm="12"
          :md="8"
          :lg="6"
          v-for="module in modules"
          :key="module.title"
        >
          <el-card
            shadow="never"
            class="module-card"
            @click="navigateTo(module.route)"
          >
            <div class="module-card-inner">
              <div
                class="module-icon-wrapper"
                :style="{ backgroundColor: module.color + '18' }"
              >
                <el-icon
                  :size="48"
                  class="module-icon"
                  :style="{ color: module.color }"
                >
                  <component :is="module.icon" />
                </el-icon>
              </div>
              <h3 class="module-title">{{ module.title }}</h3>
              <p class="module-subtitle">{{ module.subtitle }}</p>
              <el-button
                type="primary"
                :style="{ backgroundColor: module.color, borderColor: module.color }"
                round
                class="module-btn"
                size="small"
              >
                进入系统 &rarr;
              </el-button>
            </div>
          </el-card>
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'
import {
  Tickets,
  Memo,
  Apple,
  IceCream,
  DataAnalysis,
} from '@element-plus/icons-vue'

const router = useRouter()

const stats = [
  { label: '待处理请购单', value: 5, color: '#e6a23c' },
  { label: '进行中订单', value: 8, color: '#409eff' },
  { label: '已完成订单', value: 23, color: '#67c23a' },
  { label: '待结算', value: 3, color: '#f56c6c' },
]

const modules = [
  {
    title: '纸箱采购',
    subtitle: '纸箱及包装材料请购与订单管理',
    icon: Tickets,
    color: '#e6a23c',
    route: '/procurement/carton/requisition',
  },
  {
    title: '办公用品采购',
    subtitle: '办公用品、耗材请购与审批',
    icon: Memo,
    color: '#3498db',
    route: '/procurement/office',
  },
  {
    title: '牧场物料采购',
    subtitle: '牧场生产物料采购与配送管理',
    icon: Apple,
    color: '#67c23a',
    route: '/procurement/ranch',
  },
  {
    title: '低温包装采购',
    subtitle: '低温冷链包装材料采购管理',
    icon: IceCream,
    color: '#8e44ad',
    route: '/procurement/cold-chain',
  },
  {
    title: '采购汇总',
    subtitle: '采购数据统计与分析报表',
    icon: DataAnalysis,
    color: '#f56c6c',
    route: '/procurement/summary',
  },
]

function navigateTo(route) {
  router.push(route)
}
</script>

<style scoped>
.dashboard {
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  margin-bottom: 28px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: #303133;
  margin: 0 0 8px 0;
}

.page-subtitle {
  font-size: 14px;
  color: #909399;
  margin: 0;
}

/* Stats Row */
.stats-row {
  margin-bottom: 32px;
}

.stat-card {
  border-radius: 8px;
  transition: transform 0.25s, box-shadow 0.25s;
  cursor: default;
}

.stat-card:hover {
  transform: translateY(-2px);
}

.stat-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 0;
}

.stat-value {
  font-size: 32px;
  font-weight: 700;
  line-height: 1.2;
}

.stat-label {
  font-size: 13px;
  color: #606266;
  margin-top: 6px;
}

/* Module Cards */
.modules-row {
  row-gap: 20px;
}

.module-card {
  border-radius: 12px;
  cursor: pointer;
  transition: transform 0.3s, box-shadow 0.3s;
  height: 100%;
  border: 1px solid #ebeef5;
}

.module-card:hover {
  transform: translateY(-6px);
  box-shadow: 0 12px 32px rgba(0, 0, 0, 0.1);
}

.module-card-inner {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 28px 16px 20px;
  min-height: 220px;
  justify-content: space-between;
}

.module-icon-wrapper {
  width: 88px;
  height: 88px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 16px;
  flex-shrink: 0;
}

.module-icon {
  flex-shrink: 0;
}

.module-title {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
  margin: 0 0 6px 0;
}

.module-subtitle {
  font-size: 13px;
  color: #909399;
  margin: 0 0 16px 0;
  line-height: 1.4;
}

.module-btn {
  flex-shrink: 0;
}
</style>
