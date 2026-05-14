<template>
  <div class="procurement-container">
    <div class="page-header">
      <h1 class="page-title">纸箱采购申请单</h1>
      <p class="page-subtitle">查看和管理纸箱采购需求</p>
    </div>

    <!-- Search Card -->
    <el-card shadow="never" class="search-card">
      <el-form :inline="true" :model="searchForm" label-width="80">
        <el-form-item label="申请单号">
          <el-input
            v-model="searchForm.reqNo"
            placeholder="请输入申请单号"
            clearable
            style="width: 200px"
          />
        </el-form-item>
        <el-form-item label="状态">
          <el-select
            v-model="searchForm.status"
            placeholder="请选择状态"
            clearable
            style="width: 140px"
          >
            <el-option label="全部" value="" />
            <el-option label="待处理" value="待处理" />
            <el-option label="已处理" value="已处理" />
          </el-select>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleSearch">
            <el-icon><Search /></el-icon>
            查询
          </el-button>
          <el-button @click="resetSearch">
            <el-icon><Refresh /></el-icon>
            重置
          </el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- Table Card -->
    <el-card shadow="never" class="table-card">
      <el-table :data="filteredList" stripe border style="width: 100%">
        <el-table-column type="index" label="#" width="60" align="center" />
        <el-table-column prop="req_no" label="申请单号" min-width="190">
          <template #default="{ row }">
            <span class="code-text">{{ row.req_no }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="dept" label="部门" width="110" />
        <el-table-column prop="applicant" label="申请人" width="100" />
        <el-table-column prop="total_qty" label="总数" width="120" align="right">
          <template #default="{ row }">
            {{ row.total_qty.toLocaleString() }}
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="110" align="center">
          <template #default="{ row }">
            <el-tag :type="statusTagType(row.status)" effect="plain" round>
              {{ row.status }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="push_time" label="推送时间" width="180" />
        <el-table-column label="操作" width="300" fixed="right" align="center">
          <template #default="{ row }">
            <el-button type="primary" link size="small" @click="openDetail(row)">
              <el-icon><View /></el-icon>
              详情查看
            </el-button>
            <el-button
              v-if="row.status === '待处理'"
              type="success"
              link
              size="small"
              @click="createOrder(row)"
            >
              <el-icon><Plus /></el-icon>
              创建采购订单
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- Empty State -->
      <el-empty v-if="filteredList.length === 0" description="暂无匹配的申请单" />
    </el-card>

    <!-- Detail Drawer -->
    <el-drawer
      v-model="drawerVisible"
      title="申请单详情"
      size="600px"
      :before-close="closeDrawer"
    >
      <template v-if="currentRequisition">
        <!-- Basic Info -->
        <el-descriptions
          title="基本信息"
          :column="2"
          border
          size="small"
        >
          <el-descriptions-item label="申请单号" :span="2">
            <span class="code-text">{{ currentRequisition.req_no }}</span>
          </el-descriptions-item>
          <el-descriptions-item label="部门">
            {{ currentRequisition.dept }}
          </el-descriptions-item>
          <el-descriptions-item label="申请人">
            {{ currentRequisition.applicant }}
          </el-descriptions-item>
          <el-descriptions-item label="总数">
            {{ currentRequisition.total_qty.toLocaleString() }}
          </el-descriptions-item>
          <el-descriptions-item label="状态">
            <el-tag :type="statusTagType(currentRequisition.status)" effect="plain" size="small">
              {{ currentRequisition.status }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="推送时间">
            {{ currentRequisition.push_time }}
          </el-descriptions-item>
        </el-descriptions>

        <!-- Material Items -->
        <h3 class="section-title">物料清单</h3>
        <el-table :data="detailMaterials" border stripe size="small">
          <el-table-column type="index" label="#" width="50" align="center" />
          <el-table-column prop="material_code" label="物料编码" width="110" />
          <el-table-column prop="material_name" label="物料名称" min-width="200" show-overflow-tooltip />
          <el-table-column prop="spec" label="规格" min-width="180" show-overflow-tooltip />
          <el-table-column prop="qty" label="数量" width="90" align="right">
            <template #default="{ row }">
              {{ row.qty.toLocaleString() }}
            </template>
          </el-table-column>
          <el-table-column prop="unit" label="单位" width="60" align="center" />
          <el-table-column prop="is_new_product" label="新品" width="60" align="center">
            <template #default="{ row }">
              <el-tag :type="row.is_new_product ? 'danger' : 'info'" size="small" effect="dark">
                {{ row.is_new_product ? '是' : '否' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column label="OA价" width="90" align="right">
            <template #default="{ row }">
              <span v-if="row.oa_price !== null && row.oa_price !== undefined">
                ¥{{ row.oa_price.toFixed(2) }}
              </span>
              <span v-else class="no-price">-</span>
            </template>
          </el-table-column>
        </el-table>
      </template>

      <template #footer>
        <el-button @click="drawerVisible = false">关闭</el-button>
      </template>
    </el-drawer>
  </div>
</template>

<script setup>
import { ref, reactive, computed } from 'vue'
import { useRouter } from 'vue-router'
import { Search, Refresh, View, Plus } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

const router = useRouter()

// ── Mock Data ──────────────────────────────────────────────

const requisitions = ref([
  {
    id: 1,
    req_no: 'REQ20260514001',
    dept: '生产部',
    applicant: '赵工',
    total_qty: 85000,
    status: '待处理',
    push_time: '2026-05-14 09:30',
  },
  {
    id: 2,
    req_no: 'REQ20260513002',
    dept: '物流部',
    applicant: '钱工',
    total_qty: 32000,
    status: '待处理',
    push_time: '2026-05-13 14:20',
  },
  {
    id: 3,
    req_no: 'REQ20260512003',
    dept: '生产部',
    applicant: '孙工',
    total_qty: 18000,
    status: '已处理',
    push_time: '2026-05-12 10:00',
  },
])

const detailMaterials = [
  {
    material_code: 'CTN-001',
    material_name: '三层瓦楞纸箱 400x300x250mm',
    spec: '400×300×250mm B楞',
    qty: 30000,
    unit: '个',
    is_new_product: false,
    oa_price: null,
  },
  {
    material_code: 'CTN-002',
    material_name: '五层瓦楞纸箱 600x400x350mm',
    spec: '600×400×350mm BC楞',
    qty: 25000,
    unit: '个',
    is_new_product: false,
    oa_price: null,
  },
  {
    material_code: 'CTN-003',
    material_name: '重型纸箱 800x600x500mm',
    spec: '800×600×500mm 三A楞',
    qty: 15000,
    unit: '个',
    is_new_product: true,
    oa_price: 12.80,
  },
  {
    material_code: 'CTN-004',
    material_name: '食品级纸箱 500x350x300mm',
    spec: '500×350×300mm E楞 食品级',
    qty: 10000,
    unit: '个',
    is_new_product: false,
    oa_price: null,
  },
  {
    material_code: 'CTN-005',
    material_name: '防水纸箱 450x300x280mm',
    spec: '450×300×280mm 防水处理',
    qty: 5000,
    unit: '个',
    is_new_product: true,
    oa_price: 9.50,
  },
]

// ── Search State ───────────────────────────────────────────

const searchForm = reactive({
  reqNo: '',
  status: '',
})

const filteredList = ref([...requisitions.value])

function handleSearch() {
  filteredList.value = requisitions.value.filter((item) => {
    const matchReqNo =
      !searchForm.reqNo ||
      item.req_no.toLowerCase().includes(searchForm.reqNo.toLowerCase())
    const matchStatus =
      !searchForm.status || item.status === searchForm.status
    return matchReqNo && matchStatus
  })
}

function resetSearch() {
  searchForm.reqNo = ''
  searchForm.status = ''
  filteredList.value = [...requisitions.value]
}

// ── Helper ─────────────────────────────────────────────────

function statusTagType(status) {
  if (status === '待处理') return 'warning'
  if (status === '已处理') return 'success'
  return 'info'
}

// ── Detail Drawer ──────────────────────────────────────────

const drawerVisible = ref(false)
const currentRequisition = ref(null)

function openDetail(row) {
  currentRequisition.value = row
  drawerVisible.value = true
}

function closeDrawer() {
  drawerVisible.value = false
}

// ── Navigation ─────────────────────────────────────────────

function createOrder(row) {
  ElMessage.info(`正在跳转至采购订单创建...`)
  router.push(`/procurement/carton/create-order/${row.id}`)
}
</script>

<style scoped>
.procurement-container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 20px;
}

.page-header {
  margin-bottom: 24px;
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

.search-card {
  margin-bottom: 16px;
  border-radius: 8px;
}

.table-card {
  border-radius: 8px;
}

.code-text {
  font-family: 'Courier New', Courier, monospace;
  font-weight: 600;
  color: var(--el-color-primary);
  letter-spacing: 0.5px;
}

.no-price {
  color: #c0c4cc;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  color: #303133;
  margin: 24px 0 16px;
  padding-left: 8px;
  border-left: 3px solid var(--el-color-primary);
}

.el-drawer__footer {
  border-top: 1px solid #f0f0f0;
  padding: 16px 20px;
  text-align: right;
}
</style>
