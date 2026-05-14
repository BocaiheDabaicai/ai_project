<template>
  <div class="order-list">
    <!-- Page Header -->
    <div class="page-header">
      <h2>纸箱采购订单</h2>
    </div>

    <!-- Stats Row -->
    <el-row :gutter="20" class="stats-row">
      <el-col :span="4" v-for="s in statCards" :key="s.label">
        <el-card shadow="hover" :body-style="{ padding: '16px' }"
          :class="{ 'stat-active': computedStatusFilter === s.value }" @click="setStatusFilter(s.value)">
          <div class="stat-value">{{ s.count }}</div>
          <div class="stat-label">{{ s.label }}</div>
        </el-card>
      </el-col>
    </el-row>

    <!-- Search Section -->
    <el-card class="search-card" shadow="never">
      <el-form :model="searchForm" inline>
        <el-form-item label="订单编号">
          <el-input v-model="searchForm.order_no" placeholder="请输入订单编号" clearable style="width: 200px" />
        </el-form-item>
        <el-form-item label="订单状态">
          <el-select v-model="searchForm.status" placeholder="请选择状态" clearable style="width: 150px">
            <el-option label="已下单" value="已下单" />
            <el-option label="已发货" value="已发货" />
            <el-option label="已收货" value="已收货" />
            <el-option label="已结算" value="已结算" />
            <el-option label="待审批" value="待审批" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="doSearch">查询</el-button>
          <el-button @click="resetSearch">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- Table -->
    <el-card shadow="never">
      <el-table :data="filteredOrders" border stripe style="width: 100%">
        <el-table-column type="index" label="#" width="60" align="center" />
        <el-table-column prop="order_no" label="订单编号" min-width="170">
          <template #default="{ row }">
            <span class="code-text">{{ row.order_no }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="name" label="订单名称" min-width="150" show-overflow-tooltip />
        <el-table-column prop="total_qty" label="总数量" width="100" align="right">
          <template #default="{ row }">
            {{ row.total_qty.toLocaleString() }}
          </template>
        </el-table-column>
        <el-table-column prop="supplier_count" label="供应商数" width="90" align="center" />
        <el-table-column label="状态" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="statusTypeMap[row.status] || ''" size="small" effect="dark">
              {{ row.status }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="created_at" label="创建时间" width="175" align="center" />
        <el-table-column label="操作" width="100" fixed="right" align="center">
          <template #default="{ row }">
            <el-button type="primary" link size="small" @click="viewDetail(row.id)">
              详情
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<script setup>
import { ref, reactive, computed } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

// --- Mock Data ---
const orders = ref([
  {
    id: 1,
    order_no: 'PO20260514001',
    req_no: 'REQ20260514001',
    name: '纸箱批量采购',
    total_qty: 85000,
    total_delivered: 45000,
    total_amount: 268000,
    status: '已下单',
    approved: true,
    created_at: '2026-05-14 10:30',
    supplier_count: 3
  },
  {
    id: 2,
    order_no: 'PO20260513002',
    req_no: 'REQ20260513002',
    name: '物流纸箱采购',
    total_qty: 32000,
    total_delivered: 32000,
    total_amount: 96000,
    status: '已结算',
    approved: true,
    created_at: '2026-05-13 15:20',
    supplier_count: 2
  },
  {
    id: 3,
    order_no: 'PO20260512001',
    req_no: 'REQ20260512001',
    name: '电商纸箱补货',
    total_qty: 15000,
    total_delivered: 8000,
    total_amount: 52000,
    status: '已发货',
    approved: true,
    created_at: '2026-05-12 09:00',
    supplier_count: 1
  },
  {
    id: 4,
    order_no: 'PO20260511001',
    req_no: 'REQ20260511001',
    name: '出口专用纸箱',
    total_qty: 28000,
    total_delivered: 28000,
    total_amount: 156000,
    status: '已收货',
    approved: true,
    created_at: '2026-05-11 14:00',
    supplier_count: 2
  },
  {
    id: 5,
    order_no: 'PO20260510001',
    req_no: 'REQ20260510001',
    name: '新品纸箱试制',
    total_qty: 5000,
    total_delivered: 0,
    total_amount: 45000,
    status: '待审批',
    approved: false,
    created_at: '2026-05-10 11:30',
    supplier_count: 1
  }
])

// --- Status color mapping ---
const statusTypeMap = {
  '已下单': 'warning',
  '已发货': 'primary',
  '已收货': 'success',
  '已结算': 'info',
  '待审批': 'danger'
}

// --- Stats computed ---
const statCards = computed(() => {
  const allOrders = orders.value
  return [
    { label: '全部订单', value: '', count: allOrders.length, type: '' },
    { label: '已下单', value: '已下单', count: allOrders.filter(o => o.status === '已下单').length, type: 'warning' },
    { label: '已发货', value: '已发货', count: allOrders.filter(o => o.status === '已发货').length, type: 'primary' },
    { label: '已收货', value: '已收货', count: allOrders.filter(o => o.status === '已收货').length, type: 'success' },
    { label: '已结算', value: '已结算', count: allOrders.filter(o => o.status === '已结算').length, type: 'info' }
  ]
})

// --- Search form ---
const searchForm = reactive({
  order_no: '',
  status: ''
})

const computedStatusFilter = ref('')

const filteredOrders = computed(() => {
  return orders.value.filter(o => {
    const matchesNo = !searchForm.order_no || o.order_no.includes(searchForm.order_no)
    const matchesStatus = !computedStatusFilter.value || o.status === computedStatusFilter.value
    return matchesNo && matchesStatus
  })
})

// --- Methods ---
const setStatusFilter = (statusVal) => {
  computedStatusFilter.value = statusVal
  searchForm.status = statusVal
}

const doSearch = () => {
  computedStatusFilter.value = searchForm.status
}

const resetSearch = () => {
  searchForm.order_no = ''
  searchForm.status = ''
  computedStatusFilter.value = ''
}

const viewDetail = (id) => {
  router.push(`/procurement/carton/order-detail/${id}`)
}
</script>

<style scoped>
.order-list {
  padding: 20px;
}

.page-header {
  margin-bottom: 20px;
}

.page-header h2 {
  margin: 0;
  font-size: 22px;
  font-weight: 600;
}

.stats-row {
  margin-bottom: 20px;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  color: #303133;
  text-align: center;
}

.stat-label {
  font-size: 13px;
  color: #909399;
  text-align: center;
  margin-top: 6px;
}

.stat-active {
  border: 2px solid var(--el-color-primary);
}

.search-card {
  margin-bottom: 16px;
}

.code-text {
  color: var(--el-color-primary);
  font-family: monospace;
  font-weight: 500;
}
</style>
