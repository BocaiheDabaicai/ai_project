<template>
  <div class="page-container">
    <div class="summary-page">
      <div class="page-header">
        <h1 class="page-title">采购汇总</h1>
        <p class="page-subtitle">模块开发中，以下为概览数据</p>
      </div>

      <!-- Stats Cards -->
      <el-row :gutter="20" class="summary-stats">
        <el-col
          :xs="12"
          :sm="12"
          :md="6"
          v-for="stat in summaryStats"
          :key="stat.label"
        >
          <el-card shadow="hover" class="summary-stat-card">
            <div class="summary-stat-inner">
              <div
                class="summary-stat-icon"
                :style="{ backgroundColor: stat.color + '15' }"
              >
                <el-icon :size="28" :style="{ color: stat.color }">
                  <component :is="stat.icon" />
                </el-icon>
              </div>
              <div class="summary-stat-body">
                <div class="summary-stat-value" :style="{ color: stat.color }">
                  {{ stat.value }}
                </div>
                <div class="summary-stat-label">{{ stat.label }}</div>
              </div>
            </div>
          </el-card>
        </el-col>
      </el-row>

      <!-- Placeholder Notice -->
      <div class="placeholder-notice">
        <div class="icon-wrapper" style="background-color: #f56c6c18">
          <el-icon :size="56" class="notice-icon" style="color: #f56c6c">
            <DataAnalysis />
          </el-icon>
        </div>
        <h2 class="notice-title">采购汇总</h2>
        <p class="notice-subtitle">模块开发中，敬请期待...</p>
        <el-button
          type="primary"
          round
          class="back-btn"
          @click="goBack"
        >
          返回采购中心
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'
import {
  DataAnalysis,
  Coin,
  ShoppingCart,
  UserFilled,
  TrendCharts,
} from '@element-plus/icons-vue'

const router = useRouter()

const summaryStats = [
  { label: '总采购金额', value: '¥12,580,000', icon: Coin, color: '#f56c6c' },
  { label: '总订单数', value: 47, icon: ShoppingCart, color: '#409eff' },
  { label: '合作供应商', value: 12, icon: UserFilled, color: '#67c23a' },
  { label: '完成率', value: '92%', icon: TrendCharts, color: '#e6a23c' },
]

function goBack() {
  router.push('/procurement')
}
</script>

<style scoped>
.summary-page {
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

/* Summary Stats */
.summary-stats {
  margin-bottom: 48px;
}

.summary-stat-card {
  border-radius: 10px;
  transition: transform 0.25s, box-shadow 0.25s;
}

.summary-stat-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
}

.summary-stat-inner {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 0;
}

.summary-stat-icon {
  width: 56px;
  height: 56px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.summary-stat-body {
  display: flex;
  flex-direction: column;
}

.summary-stat-value {
  font-size: 22px;
  font-weight: 700;
  line-height: 1.3;
  white-space: nowrap;
}

.summary-stat-label {
  font-size: 13px;
  color: #606266;
  margin-top: 2px;
}

/* Placeholder */
.placeholder-notice {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  background: #fff;
  border-radius: 16px;
  padding: 56px 80px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.06);
}

.icon-wrapper {
  width: 108px;
  height: 108px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 20px;
}

.notice-icon {
  flex-shrink: 0;
}

.notice-title {
  font-size: 22px;
  font-weight: 600;
  color: #303133;
  margin: 0 0 8px 0;
}

.notice-subtitle {
  font-size: 15px;
  color: #909399;
  margin: 0 0 28px 0;
}

.back-btn {
  padding: 10px 36px;
  font-size: 15px;
}
</style>
