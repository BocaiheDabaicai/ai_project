<template>
  <div class="page-container">
    <div class="placeholder-page">
      <div class="placeholder-card">
        <div class="icon-wrapper" style="background-color: #3498db18">
          <el-icon :size="64" class="placeholder-icon" style="color: #3498db">
            <Memo />
          </el-icon>
        </div>
        <h1 class="placeholder-title">办公用品采购</h1>
        <p class="placeholder-subtitle">模块开发中，敬请期待...</p>
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
import { Memo } from '@element-plus/icons-vue'

const router = useRouter()

function goBack() {
  router.push('/procurement')
}
</script>

<style scoped>
.placeholder-page {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: calc(100vh - 140px);
}

.placeholder-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  background: #fff;
  border-radius: 16px;
  padding: 60px 80px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.06);
}

.icon-wrapper {
  width: 120px;
  height: 120px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 24px;
}

.placeholder-icon {
  flex-shrink: 0;
}

.placeholder-title {
  font-size: 26px;
  font-weight: 600;
  color: #303133;
  margin: 0 0 12px 0;
}

.placeholder-subtitle {
  font-size: 16px;
  color: #909399;
  margin: 0 0 32px 0;
}

.back-btn {
  padding: 10px 36px;
  font-size: 15px;
}
</style>
