<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const orders = ref([
  { id: 1, orderNo: 'PO20260510001', reqNo: 'REQ20260510008', name: '生产物料紧急采购', totalQty: 2800, status: '已下单', supplierCount: 2, createTime: '2026-05-10 10:30' },
  { id: 2, orderNo: 'PO20260509002', reqNo: 'REQ20260509007', name: '研发电子元器件采购', totalQty: 500, status: '已发货', supplierCount: 1, createTime: '2026-05-09 15:20' },
  { id: 3, orderNo: 'PO20260508003', reqNo: 'REQ20260508006', name: '螺栓紧固件批量采购', totalQty: 5000, status: '已收货', supplierCount: 3, createTime: '2026-05-08 11:00' },
  { id: 4, orderNo: 'PO20260507004', reqNo: 'REQ20260507004', name: '电缆线材采购', totalQty: 800, status: '已结算', supplierCount: 2, createTime: '2026-05-07 09:15' },
  { id: 5, orderNo: 'PO20260506005', reqNo: 'REQ20260506003', name: '金属板材补充采购', totalQty: 1200, status: '已下单', supplierCount: 1, createTime: '2026-05-06 14:45' },
])

const statusFilter = ref('')

const filteredOrders = computed(() => {
  if (!statusFilter.value) return orders.value
  return orders.value.filter(o => o.status === statusFilter.value)
})

function statusType(s) {
  return { '已下单': 'warning', '已发货': '', '已收货': 'success', '已结算': 'info' }[s] || 'info'
}

function viewDetail(id) {
  router.push(`/procurement/order-detail/${id}`)
}
</script>

<template>
  <div class="page-container">
    <h2 class="page-title">
      <el-icon :size="20" color="var(--color-accent)"><Document /></el-icon>
      采购订单管理
    </h2>

    <!-- 搜索栏 -->
    <div class="search-bar">
      <el-input placeholder="订单编号/名称" clearable style="width:200px" />
      <el-select v-model="statusFilter" placeholder="全部状态" clearable style="width:130px">
        <el-option label="已下单" value="已下单" />
        <el-option label="已发货" value="已发货" />
        <el-option label="已收货" value="已收货" />
        <el-option label="已结算" value="已结算" />
      </el-select>
      <el-button type="primary" :icon="Search">查询</el-button>
      <el-button :icon="RefreshRight" @click="statusFilter = ''">重置</el-button>
    </div>

    <!-- 统计 -->
    <div class="stats-row">
      <div class="stat-card"><div class="stat-value">{{ orders.length }}</div><div class="stat-label">全部订单</div></div>
      <div class="stat-card"><div class="stat-value" style="color:#e6a23c">{{ orders.filter(o => o.status === '已下单').length }}</div><div class="stat-label">已下单</div></div>
      <div class="stat-card"><div class="stat-value" style="color:var(--color-accent)">{{ orders.filter(o => o.status === '已发货').length }}</div><div class="stat-label">已发货</div></div>
      <div class="stat-card"><div class="stat-value" style="color:#67c23a">{{ orders.filter(o => o.status === '已收货').length }}</div><div class="stat-label">已收货</div></div>
      <div class="stat-card"><div class="stat-value">{{ orders.filter(o => o.status === '已结算').length }}</div><div class="stat-label">已结算</div></div>
    </div>

    <!-- 表格 -->
    <div class="table-wrapper">
      <table class="data-table">
        <thead>
          <tr>
            <th>#</th>
            <th>订单编号</th>
            <th>来源请购单</th>
            <th>订单名称</th>
            <th>总计数量</th>
            <th>供应商数</th>
            <th>状态</th>
            <th>创建时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(item, idx) in filteredOrders" :key="item.id">
            <td>{{ idx + 1 }}</td>
            <td><span class="code-text">{{ item.orderNo }}</span></td>
            <td><span class="code-text">{{ item.reqNo }}</span></td>
            <td>{{ item.name }}</td>
            <td><strong>{{ item.totalQty.toLocaleString() }}</strong></td>
            <td>{{ item.supplierCount }} 家</td>
            <td>
              <el-tag :type="statusType(item.status)" effect="light" size="small" round>{{ item.status }}</el-tag>
            </td>
            <td>{{ item.createTime }}</td>
            <td>
              <el-button type="primary" link size="small" @click="viewDetail(item.id)">详情</el-button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.code-text {
  font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  color: var(--color-accent);
}
</style>
