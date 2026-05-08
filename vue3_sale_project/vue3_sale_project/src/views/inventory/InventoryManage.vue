<script setup>
import { ref } from 'vue'

const items = ref([
  { id: 1, sku: 'MAT-001', name: '不锈钢板材 304/2mm', category: '金属材料', stock: 1250, unit: 'kg', safeStock: 500, warehouse: 'A1仓库', updateTime: '2026-05-07 16:30' },
  { id: 2, sku: 'MAT-002', name: '电子芯片 ATmega328P', category: '电子元器件', stock: 3800, unit: 'pcs', safeStock: 1000, warehouse: 'B2仓库', updateTime: '2026-05-06 10:00' },
  { id: 3, sku: 'MAT-003', name: '高强度螺栓 M12x50', category: '紧固件', stock: 560, unit: 'box', safeStock: 300, warehouse: 'A2仓库', updateTime: '2026-05-07 09:15' },
  { id: 4, sku: 'MAT-004', name: '工业润滑油 L-HM46', category: '化工辅料', stock: 320, unit: '桶', safeStock: 200, warehouse: 'C1仓库', updateTime: '2026-05-05 14:00' },
  { id: 5, sku: 'MAT-005', name: '铜芯电缆 3x2.5mm²', category: '电气材料', stock: 80, unit: '卷', safeStock: 100, warehouse: 'A1仓库', updateTime: '2026-05-07 08:00' }
])

const totalStock = items.value.reduce((sum, i) => sum + i.stock, 0)
const lowStockCount = items.value.filter(i => i.stock < i.safeStock).length
</script>

<template>
  <div class="page-container">
    <h2 class="page-title">
      <el-icon :size="20" color="var(--color-accent)"><Box /></el-icon>
      库存管理
    </h2>

    <!-- 统计卡片 -->
    <div class="stats-row">
      <div class="stat-card">
        <div class="stat-value">{{ items.length }}</div>
        <div class="stat-label">物料种类</div>
      </div>
      <div class="stat-card">
        <div class="stat-value" style="color:#f56c6c">{{ lowStockCount }}</div>
        <div class="stat-label">低于安全库存</div>
      </div>
      <div class="stat-card">
        <div class="stat-value" style="color:var(--color-accent)">{{ totalStock.toLocaleString() }}</div>
        <div class="stat-label">库存总量</div>
      </div>
      <el-button type="success" :icon="Plus" style="align-self:center;height:40px">入库登记</el-button>
      <el-button type="warning" :icon="Minus" style="align-self:center;height:40px">出库登记</el-button>
    </div>

    <!-- 数据表格 -->
    <div class="table-wrapper">
      <table class="data-table">
        <thead>
          <tr>
            <th>#</th>
            <th>SKU编码</th>
            <th>物料名称</th>
            <th>类别</th>
            <th>当前库存</th>
            <th>单位</th>
            <th>安全库存</th>
            <th>所在仓库</th>
            <th>库存状态</th>
            <th>更新时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(item, idx) in items" :key="item.id">
            <td>{{ idx + 1 }}</td>
            <td><span class="sku-text">{{ item.sku }}</span></td>
            <td>{{ item.name }}</td>
            <td>{{ item.category }}</td>
            <td>
              <span :style="{ fontWeight: 600, color: item.stock < item.safeStock ? '#f56c6c' : 'var(--text-primary)' }">
                {{ item.stock.toLocaleString() }}
              </span>
            </td>
            <td>{{ item.unit }}</td>
            <td>{{ item.safeStock.toLocaleString() }}</td>
            <td>{{ item.warehouse }}</td>
            <td>
              <el-tag v-if="item.stock < item.safeStock" type="danger" effect="light" size="small" round>库存不足</el-tag>
              <el-tag v-else type="success" effect="light" size="small" round>正常</el-tag>
            </td>
            <td>{{ item.updateTime }}</td>
            <td>
              <el-button type="primary" link size="small">查看</el-button>
              <el-button type="primary" link size="small">调整</el-button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.sku-text {
  font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  color: var(--color-accent);
}
</style>
