<template>
  <div class="order-detail">
    <!-- Breadcrumb -->
    <el-breadcrumb separator="/" class="breadcrumb">
      <el-breadcrumb-item :to="{ path: '/procurement/carton/order-list' }">纸箱采购订单</el-breadcrumb-item>
      <el-breadcrumb-item>订单详情</el-breadcrumb-item>
    </el-breadcrumb>

    <!-- ======== A. Order Info ======== -->
    <el-card class="section-card" shadow="never">
      <template #header>
        <div class="card-header">
          <span>订单信息</span>
          <el-tag v-if="order.is_new" type="warning" size="small" effect="dark">新单</el-tag>
        </div>
      </template>
      <el-descriptions :column="3" border>
        <el-descriptions-item label="订单编号" width="150">{{ order.order_no }}</el-descriptions-item>
        <el-descriptions-item label="关联申请单" width="150">{{ order.req_no }}</el-descriptions-item>
        <el-descriptions-item label="订单名称">{{ order.name }}</el-descriptions-item>
        <el-descriptions-item label="总数量">{{ order.total_qty.toLocaleString() }} 个</el-descriptions-item>
        <el-descriptions-item label="已交付">{{ order.total_delivered.toLocaleString() }} 个</el-descriptions-item>
        <el-descriptions-item label="剩余数量">
          <span :class="orderRemaining > 0 ? 'text-danger' : 'text-success'">
            {{ orderRemaining.toLocaleString() }} 个
          </span>
        </el-descriptions-item>
        <el-descriptions-item label="订单状态">
          <el-tag :type="statusType(order.status)" size="small" effect="dark">{{ order.status }}</el-tag>
        </el-descriptions-item>
        <el-descriptions-item label="审批状态">
          <el-tag :type="order.approved ? 'success' : 'danger'" size="small">
            {{ order.approved ? '已审批' : '待审批' }}
          </el-tag>
        </el-descriptions-item>
        <el-descriptions-item label="创建时间">{{ order.created_at }}</el-descriptions-item>
        <el-descriptions-item label="订单金额" :span="3">
          <span class="text-amount">¥{{ order.total_amount.toLocaleString() }}</span>
        </el-descriptions-item>
      </el-descriptions>
    </el-card>

    <!-- ======== B. Action Buttons ======== -->
    <div class="action-bar">
      <!-- 已下单: 发货 + 撤回订单 -->
      <template v-if="order.status === '已下单'">
        <el-button type="primary" :icon="Upload" @click="openShipDialog">发货</el-button>
        <el-button type="danger" :icon="Delete" @click="handleWithdraw">撤回订单</el-button>
      </template>

      <!-- 已发货: 退货 -->
      <template v-if="order.status === '已发货'">
        <el-button type="warning" :icon="Download" @click="handleReturn">退货</el-button>
      </template>

      <!-- 已收货: 确认结算 for each supplier 已验收 -->
      <template v-if="order.status === '已收货'">
        <el-button
          v-for="sup in eligibleSettleSuppliers"
          :key="sup.code"
          type="success"
          :icon="Check"
          size="small"
          @click="handleSupplierSettle(sup.code, sup.name)"
        >
          确认结算 — {{ sup.name }}
        </el-button>
      </template>

      <!-- Always: 返回 -->
      <el-button :icon="ArrowLeft" @click="goBack">返回</el-button>
    </div>

    <!-- ======== C. Material Groups ======== -->
    <div v-for="item in items" :key="item.material_code" class="material-group">
      <el-card shadow="never">
        <!-- Card Header -->
        <template #header>
          <div class="material-header">
            <div class="material-info">
              <span class="material-name">{{ item.material_name }}</span>
              <span class="material-code">({{ item.material_code }})</span>
              <span v-if="item.spec" class="material-spec">— {{ item.spec }}</span>
              <el-tag v-if="item.is_new" type="warning" size="small" effect="plain" class="ml-8">新品</el-tag>
            </div>
            <div class="material-meta">
              <span class="meta-item">总量: <b>{{ item.total_qty.toLocaleString() }}</b> 个</span>
              <span class="meta-item">策略: <b>{{ item.strategy }}</b></span>
              <span class="meta-item">单价: <b>¥{{ item.price.toFixed(2) }}</b></span>
              <el-tag v-if="item.pkg" size="small" type="info" effect="plain">{{ item.pkg }}</el-tag>
              <span v-if="item.oa_price" class="meta-item oa-price">OA价: ¥{{ item.oa_price.toFixed(2) }}</span>
            </div>
          </div>
        </template>

        <!-- Supplier Sub-Table -->
        <el-table :data="item.suppliers" border stripe size="small" style="width: 100%">
          <el-table-column label="供应商" min-width="140">
            <template #default="{ row }">
              <div class="supplier-cell">
                <span class="supplier-code">{{ row.code }}</span>
                <span class="supplier-name">{{ row.name }}</span>
              </div>
            </template>
          </el-table-column>
          <el-table-column prop="pkg" label="包型" width="80" align="center">
            <template #default="{ row }">
              <el-tag v-if="row.pkg" size="small" type="info" effect="plain">{{ row.pkg }}</el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="allocated_qty" label="分配数量" width="100" align="right">
            <template #default="{ row }">{{ row.allocated_qty.toLocaleString() }}</template>
          </el-table-column>
          <el-table-column prop="delivered_qty" label="已交付" width="90" align="right">
            <template #default="{ row }">{{ row.delivered_qty.toLocaleString() }}</template>
          </el-table-column>
          <el-table-column label="剩余" width="80" align="right">
            <template #default="{ row }">
              <span :class="computedRemaining(row) > 0 ? 'text-danger' : 'text-success'">
                {{ computedRemaining(row) }}
              </span>
            </template>
          </el-table-column>
          <el-table-column prop="unit_price" label="单价" width="80" align="right">
            <template #default="{ row }">¥{{ row.unit_price.toFixed(2) }}</template>
          </el-table-column>
          <el-table-column label="小计" width="100" align="right">
            <template #default="{ row }">
              ¥{{ (row.delivered_qty * row.unit_price).toLocaleString() }}
            </template>
          </el-table-column>
          <el-table-column label="色差" width="85" align="center">
            <template #default="{ row }">
              <el-tag
                :type="row.color_diff === '无色差' ? 'success' : 'warning'"
                size="small"
                effect="plain"
              >{{ row.color_diff }}</el-tag>
            </template>
          </el-table-column>
          <el-table-column label="质量" width="80" align="center">
            <template #default="{ row }">
              <el-tag
                :type="row.quality === '非常好' ? 'success' : 'primary'"
                size="small"
                effect="plain"
              >{{ row.quality }}</el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="score" label="评分" width="65" align="center">
            <template #default="{ row }">
              <el-rate :model-value="row.score" disabled show-score text-color="#ff9900" score-template="{value}" />
            </template>
          </el-table-column>
          <el-table-column label="标准样品" width="80" align="center">
            <template #default="{ row }">
              <el-tag :type="row.is_standard_sample ? 'success' : 'info'" size="small">
                {{ row.is_standard_sample ? '是' : '否' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column label="批量历史" width="80" align="center">
            <template #default="{ row }">
              <el-tag :type="row.has_bulk_history ? 'success' : 'info'" size="small">
                {{ row.has_bulk_history ? '有' : '无' }}
              </el-tag>
            </template>
          </el-table-column>
          <el-table-column prop="location" label="所在地" width="110" align="center" />
          <el-table-column label="操作" width="140" fixed="right" align="center">
            <template #default="{ row }">
              <el-button type="primary" link size="small" @click="openShipmentsDialog(row.code, row.name, item.material_code)">
                追溯查询
              </el-button>
              <el-button
                v-if="canSettleSupplier(row.code)"
                type="success"
                link
                size="small"
                @click="handleSupplierSettle(row.code, row.name)"
              >
                结算
              </el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-card>
    </div>

    <!-- ======== D. Ship Dialog ======== -->
    <el-dialog v-model="shipDialogVisible" title="发货管理" width="900px" :top="'5vh'" destroy-on-close>
      <div class="ship-summary">
        总分配: <b>{{ order.total_qty.toLocaleString() }}</b> 个 &nbsp;|&nbsp;
        已交付: <b>{{ order.total_delivered.toLocaleString() }}</b> 个 &nbsp;|&nbsp;
        剩余: <b class="text-danger">{{ orderRemaining.toLocaleString() }}</b> 个
      </div>

      <el-table :data="shipRows" border stripe size="small" style="width: 100%; margin-top: 16px;">
        <el-table-column prop="material_name" label="物料" min-width="120" />
        <el-table-column label="供应商" min-width="130">
          <template #default="{ row }">{{ row.supplier_name }}</template>
        </el-table-column>
        <el-table-column prop="total_alloc" label="分配数量" width="100" align="right">
          <template #default="{ row }">{{ row.total_alloc.toLocaleString() }}</template>
        </el-table-column>
        <el-table-column prop="delivered" label="已交付" width="90" align="right">
          <template #default="{ row }">{{ row.delivered.toLocaleString() }}</template>
        </el-table-column>
        <el-table-column prop="remaining" label="剩余" width="80" align="right">
          <template #default="{ row }">
            <span class="text-danger">{{ row.remaining }}</span>
          </template>
        </el-table-column>
        <el-table-column label="发货数量" width="140" align="center">
          <template #default="{ row }">
            <el-input-number
              v-model="row.ship_qty"
              :min="0"
              :max="row.remaining"
              :step="100"
              size="small"
              controls-position="right"
            />
          </template>
        </el-table-column>
      </el-table>

      <div class="ship-arrival">
        <span class="label">预计到货时间：</span>
        <el-date-picker v-model="shipArrivalTime" type="date" placeholder="选择到货日期"
          value-format="YYYY-MM-DD" style="width: 200px" />
      </div>

      <template #footer>
        <el-button @click="shipDialogVisible = false">取消</el-button>
        <el-button type="primary" :icon="Check" @click="confirmShip">确认发货</el-button>
      </template>
    </el-dialog>

    <!-- ======== E. Shipments Dialog (追溯) ======== -->
    <el-dialog v-model="shipmentsDialogVisible" :title="'采购子单追溯 — ' + shipmentDialogTitle"
      width="1000px" :top="'5vh'" destroy-on-close>
      <el-table :data="dialogShipments" border stripe size="small" style="width: 100%">
        <el-table-column prop="batch_no" label="批次号" min-width="180">
          <template #default="{ row }">
            <span class="code-text">{{ row.batch_no }}</span>
          </template>
        </el-table-column>
        <el-table-column prop="material_name" label="物料" min-width="130" />
        <el-table-column label="供应商" min-width="130">
          <template #default="{ row }">{{ row.supplier_name }}</template>
        </el-table-column>
        <el-table-column prop="qty" label="数量" width="80" align="right">
          <template #default="{ row }">{{ row.qty.toLocaleString() }}</template>
        </el-table-column>
        <el-table-column prop="unit" label="单位" width="60" align="center" />
        <el-table-column prop="cost" label="金额" width="100" align="right">
          <template #default="{ row }">¥{{ row.cost.toLocaleString() }}</template>
        </el-table-column>
        <el-table-column label="状态" width="90" align="center">
          <template #default="{ row }">
            <el-tag :type="statusType(row.status)" size="small" effect="dark">{{ row.status }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="arrival_time" label="到货时间" width="110" align="center" />
        <el-table-column prop="created_at" label="创建时间" width="160" align="center" />
        <el-table-column label="操作" width="100" fixed="right" align="center">
          <template #default="{ row }">
            <el-button type="primary" link size="small" @click="showShipmentDetail(row)">详情</el-button>
            <el-button
              v-if="row.status === '已下单'"
              type="primary"
              link
              size="small"
              @click="updateShipStatus(row, '已发货')"
            >标记发货</el-button>
            <el-button
              v-if="row.status === '已发货'"
              type="primary"
              link
              size="small"
              @click="updateShipStatus(row, '已入库')"
            >标记入库</el-button>
            <el-button
              v-if="row.status === '已入库'"
              type="primary"
              link
              size="small"
              @click="updateShipStatus(row, '已验收')"
            >标记验收</el-button>
            <el-button
              v-if="row.status === '已验收'"
              type="success"
              link
              size="small"
              @click="handleSettleConfirmation(row)"
            >结算</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-dialog>

    <!-- Shipment Detail Sub-Dialog -->
    <el-dialog v-model="detailDialogVisible" title="批次详情" width="600px" :top="'15vh'" destroy-on-close>
      <el-descriptions v-if="currentShipment" :column="2" border>
        <el-descriptions-item label="批次号" :span="2">{{ currentShipment.batch_no }}</el-descriptions-item>
        <el-descriptions-item label="物料编码">{{ currentShipment.material_code }}</el-descriptions-item>
        <el-descriptions-item label="物料名称">{{ currentShipment.material_name }}</el-descriptions-item>
        <el-descriptions-item label="供应商编码">{{ currentShipment.supplier_code }}</el-descriptions-item>
        <el-descriptions-item label="供应商名称">{{ currentShipment.supplier_name }}</el-descriptions-item>
        <el-descriptions-item label="数量">{{ currentShipment.qty.toLocaleString() }}</el-descriptions-item>
        <el-descriptions-item label="单位">{{ currentShipment.unit }}</el-descriptions-item>
        <el-descriptions-item label="金额">¥{{ currentShipment.cost.toLocaleString() }}</el-descriptions-item>
        <el-descriptions-item label="状态">
          <el-tag :type="statusType(currentShipment.status)" size="small">{{ currentShipment.status }}</el-tag>
        </el-descriptions-item>
        <el-descriptions-item label="到货时间">{{ currentShipment.arrival_time }}</el-descriptions-item>
        <el-descriptions-item label="创建时间" :span="2">{{ currentShipment.created_at }}</el-descriptions-item>
      </el-descriptions>
      <template #footer>
        <el-button @click="detailDialogVisible = false">关闭</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'
import { ArrowLeft, Check, Delete, Upload, Download, Search } from '@element-plus/icons-vue'

const route = useRoute()
const router = useRouter()

// ---- Order Mock ----
const order = ref({
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
  is_new: false
})

// ---- Items (material groups) ----
const items = ref([
  {
    material_code: 'CTN-001',
    material_name: '三层瓦楞纸箱',
    spec: '三层瓦楞',
    total_qty: 30000,
    strategy: '最低价',
    price: 3.05,
    pkg: '标包A',
    is_new: false,
    oa_price: null,
    suppliers: [
      { code: 'SUP002', name: '广州精工包装', allocated_qty: 20000, delivered_qty: 12000, unit_price: 3.05, pkg: '标包A', color_diff: '无色差', quality: '非常好', score: 4.8, is_standard_sample: true, has_bulk_history: true, location: '广东广州' },
      { code: 'SUP001', name: '浙江优质纸箱', allocated_qty: 10000, delivered_qty: 5000, unit_price: 3.20, pkg: '标包A', color_diff: '轻微色差', quality: '良好', score: 4.5, is_standard_sample: true, has_bulk_history: true, location: '浙江杭州' }
    ]
  },
  {
    material_code: 'CTN-002',
    material_name: '五层瓦楞纸箱',
    spec: '五层瓦楞',
    total_qty: 25000,
    strategy: '自定义',
    price: 5.80,
    pkg: '标包B',
    is_new: false,
    oa_price: null,
    suppliers: [
      { code: 'SUP001', name: '浙江优质纸箱', allocated_qty: 15000, delivered_qty: 8000, unit_price: 5.80, pkg: '标包B', color_diff: '无色差', quality: '良好', score: 4.5, is_standard_sample: true, has_bulk_history: true, location: '浙江杭州' },
      { code: 'SUP003', name: '上海环保包装', allocated_qty: 10000, delivered_qty: 5000, unit_price: 6.00, pkg: '标包B', color_diff: '轻微色差', quality: '良好', score: 4.2, is_standard_sample: true, has_bulk_history: false, location: '上海' }
    ]
  },
  {
    material_code: 'CTN-003',
    material_name: '重型纸箱',
    spec: '加强型',
    total_qty: 15000,
    strategy: '最低价',
    price: 12.50,
    pkg: '标包C',
    is_new: true,
    oa_price: 12.80,
    suppliers: [
      { code: 'SUP004', name: '北京新材料科技', allocated_qty: 15000, delivered_qty: 0, unit_price: 12.50, pkg: '标包C', color_diff: '无色差', quality: '良好', score: 4.0, is_standard_sample: false, has_bulk_history: false, location: '北京' }
    ]
  },
  {
    material_code: 'CTN-004',
    material_name: '食品级纸箱',
    spec: '食品级',
    total_qty: 10000,
    strategy: '最低价',
    price: 4.80,
    pkg: '标包D',
    is_new: false,
    oa_price: null,
    suppliers: [
      { code: 'SUP002', name: '广州精工包装', allocated_qty: 10000, delivered_qty: 10000, unit_price: 4.80, pkg: '标包D', color_diff: '无色差', quality: '非常好', score: 4.8, is_standard_sample: true, has_bulk_history: true, location: '广东广州' }
    ]
  },
  {
    material_code: 'CTN-005',
    material_name: '防水纸箱',
    spec: '防水型',
    total_qty: 5000,
    strategy: '最低价',
    price: 8.80,
    pkg: '标包E',
    is_new: true,
    oa_price: 9.50,
    suppliers: [
      { code: 'SUP005', name: '深圳精密包装', allocated_qty: 5000, delivered_qty: 5000, unit_price: 8.80, pkg: '标包E', color_diff: '无色差', quality: '良好', score: 4.3, is_standard_sample: false, has_bulk_history: false, location: '广东深圳' }
    ]
  }
])

// ---- Shipment Batches Mock ----
const shipments = ref([
  { id: 1, batch_no: 'BATCH-20260514-001', material_code: 'CTN-001', material_name: '三层瓦楞纸箱', supplier_code: 'SUP002', supplier_name: '广州精工包装', qty: 8000, unit: '个', cost: 24400, status: '已验收', arrival_time: '2026-05-16', created_at: '2026-05-14 11:00' },
  { id: 2, batch_no: 'BATCH-20260514-002', material_code: 'CTN-001', material_name: '三层瓦楞纸箱', supplier_code: 'SUP001', supplier_name: '浙江优质纸箱', qty: 5000, unit: '个', cost: 16000, status: '已入库', arrival_time: '2026-05-17', created_at: '2026-05-14 14:00' },
  { id: 3, batch_no: 'BATCH-20260515-001', material_code: 'CTN-002', material_name: '五层瓦楞纸箱', supplier_code: 'SUP001', supplier_name: '浙江优质纸箱', qty: 8000, unit: '个', cost: 46400, status: '已发货', arrival_time: '2026-05-18', created_at: '2026-05-15 09:00' },
  { id: 4, batch_no: 'BATCH-20260515-002', material_code: 'CTN-002', material_name: '五层瓦楞纸箱', supplier_code: 'SUP003', supplier_name: '上海环保包装', qty: 5000, unit: '个', cost: 30000, status: '已下单', arrival_time: '2026-05-19', created_at: '2026-05-15 10:30' },
  { id: 5, batch_no: 'BATCH-20260514-006', material_code: 'CTN-001', material_name: '三层瓦楞纸箱', supplier_code: 'SUP002', supplier_name: '广州精工包装', qty: 4000, unit: '个', cost: 12200, status: '已入库', arrival_time: '2026-05-17', created_at: '2026-05-14 13:00' },
  { id: 6, batch_no: 'BATCH-20260514-003', material_code: 'CTN-004', material_name: '食品级纸箱', supplier_code: 'SUP002', supplier_name: '广州精工包装', qty: 10000, unit: '个', cost: 48000, status: '已验收', arrival_time: '2026-05-15', created_at: '2026-05-14 15:00' },
  { id: 7, batch_no: 'BATCH-20260514-004', material_code: 'CTN-005', material_name: '防水纸箱', supplier_code: 'SUP005', supplier_name: '深圳精密包装', qty: 5000, unit: '个', cost: 44000, status: '已结算', arrival_time: '2026-05-15', created_at: '2026-05-14 16:00' }
])

// ------- computed properties -------
const orderRemaining = computed(() => {
  return order.value.total_qty - order.value.total_delivered
})

const statusType = (status) => {
  const map = {
    '已下单': 'warning',
    '已发货': 'primary',
    '已收货': 'success',
    '已结算': 'info',
    '待审批': 'danger',
    '已入库': '',
    '已验收': 'success'
  }
  return map[status] || ''
}

const computedRemaining = (supplier) => {
  return supplier.allocated_qty - supplier.delivered_qty
}

// Which suppliers have accepted shipments (for settlement eligibility at order level)
const eligibleSettleSuppliers = computed(() => {
  const result = []
  const seen = new Set()
  for (const s of shipments.value) {
    if (s.status === '已验收' && !seen.has(s.supplier_code) && canSettleSupplier(s.supplier_code)) {
      result.push({ code: s.supplier_code, name: s.supplier_name })
      seen.add(s.supplier_code)
    }
  }
  return result
})

// Check if a supplier can be settled (has accepted shipments and not all settled)
const canSettleSupplier = (supplierCode) => {
  const related = shipments.value.filter(s => s.supplier_code === supplierCode)
  if (related.length === 0) return false
  const hasAccepted = related.some(s => s.status === '已验收')
  const allSettled = related.length > 0 && related.every(s => s.status === '已结算')
  return hasAccepted && !allSettled
}

// ------- Ship Dialog (D) -------
const shipDialogVisible = ref(false)
const shipRows = ref([])
const shipArrivalTime = ref('')

const openShipDialog = () => {
  const rows = []
  for (const item of items.value) {
    for (const sup of item.suppliers) {
      const remaining = sup.allocated_qty - sup.delivered_qty
      if (remaining > 0) {
        rows.push({
          key: item.material_code + '_' + sup.code,
          material_code: item.material_code,
          material_name: item.material_name,
          supplier_code: sup.code,
          supplier_name: sup.name,
          total_alloc: sup.allocated_qty,
          delivered: sup.delivered_qty,
          remaining,
          ship_qty: 0
        })
      }
    }
  }
  shipRows.value = rows
  shipArrivalTime.value = ''
  shipDialogVisible.value = true
}

const confirmShip = () => {
  let nextId = shipments.value.length + 1
  let totalShipped = 0
  const now = new Date()
  const nowStr = now.getFullYear() +
    '-' + String(now.getMonth() + 1).padStart(2, '0') +
    '-' + String(now.getDate()).padStart(2, '0') + ' ' +
    String(now.getHours()).padStart(2, '0') + ':' +
    String(now.getMinutes()).padStart(2, '0')

  for (const row of shipRows.value) {
    if (row.ship_qty > 0) {
      const batchNo = 'BATCH-' + shipArrivalTime.value.replace(/-/g, '') || nowStr.replace(/[ :]/g, '').slice(0, 8)
      const cost = Math.round(row.ship_qty * (
        items.value
          .find(i => i.material_code === row.material_code)
          ?.suppliers.find(s => s.code === row.supplier_code)
          ?.unit_price || 0
      ))

      shipments.value.push({
        id: nextId++,
        batch_no: 'BATCH-' + nowStr.replace(/[\s:]/g, '').slice(0, 8) + '-' + String(row.ship_qty).padStart(3, '0'),
        material_code: row.material_code,
        material_name: row.material_name,
        supplier_code: row.supplier_code,
        supplier_name: row.supplier_name,
        qty: row.ship_qty,
        unit: '个',
        cost: cost,
        status: '已下单',
        arrival_time: shipArrivalTime.value || '待定',
        created_at: nowStr
      })

      // update supplier delivered_qty
      for (const item of items.value) {
        if (item.material_code === row.material_code) {
          const sup = item.suppliers.find(s => s.code === row.supplier_code)
          if (sup) {
            sup.delivered_qty += row.ship_qty
          }
        }
      }
      totalShipped += row.ship_qty
    }
  }

  // update order totals
  if (totalShipped > 0) {
    order.value.total_delivered += totalShipped
    // update status if fully delivered
    if (order.value.total_delivered >= order.value.total_qty) {
      order.value.status = '已收货'
    } else if (order.value.status === '已下单') {
      order.value.status = '已发货'
    }
  }

  shipDialogVisible.value = false
  ElMessage.success('发货成功，已创建 ' + totalShipped + ' 个批次记录')
}

// ------- Shipments Dialog (E) -------
const shipmentsDialogVisible = ref(false)
const shipmentDialogTitle = ref('')
const trackMaterialCode = ref('')
const trackSupplierCode = ref('')

const dialogShipments = computed(() => {
  return shipments.value.filter(s =>
    s.material_code === trackMaterialCode.value &&
    s.supplier_code === trackSupplierCode.value
  )
})

const openShipmentsDialog = (supplierCode, supplierName, materialCode) => {
  trackSupplierCode.value = supplierCode
  trackMaterialCode.value = materialCode
  shipmentDialogTitle.value = supplierName
  shipmentsDialogVisible.value = true
}

const updateShipStatus = (shipment, newStatus) => {
  const idx = shipments.value.findIndex(s => s.id === shipment.id)
  if (idx !== -1) {
    shipments.value[idx].status = newStatus
    // update order status if needed
    if (newStatus === '已验收' && order.value.status !== '已收货') {
      order.value.status = '已收货'
    }
    ElMessage.success('状态已更新为: ' + newStatus)
  }
}

// ------- Detail Sub-Dialog -------
const detailDialogVisible = ref(false)
const currentShipment = ref(null)

const showShipmentDetail = (shipment) => {
  currentShipment.value = { ...shipment }
  detailDialogVisible.value = true
}

// ------- Settlement -------
const handleSupplierSettle = (supplierCode, supplierName) => {
  ElMessageBox.confirm(
    '确认对供应商 "' + supplierName + '" 进行结算？结算后相关批次将标记为"已结算"。',
    '结算确认',
    { confirmButtonText: '确认结算', cancelButtonText: '取消', type: 'info' }
  ).then(() => {
    let updated = 0
    for (const s of shipments.value) {
      if (s.supplier_code === supplierCode && s.status === '已验收') {
        s.status = '已结算'
        updated++
      }
    }

    // update order status if all batches are settled
    const allSettled = shipments.value.length > 0 && shipments.value.every(s => s.status === '已结算')
    if (allSettled) {
      order.value.status = '已结算'
    }

    ElMessage.success('已为 ' + supplierName + ' 结算 ' + updated + ' 个批次')
  }).catch(() => {})
}

const handleSettleConfirmation = (shipment) => {
  ElMessageBox.confirm(
    '确认结算批次 ' + shipment.batch_no + '？金额: ¥' + shipment.cost.toLocaleString(),
    '结算确认',
    { confirmButtonText: '确认', cancelButtonText: '取消', type: 'info' }
  ).then(() => {
    const idx = shipments.value.findIndex(s => s.id === shipment.id)
    if (idx !== -1) {
      shipments.value[idx].status = '已结算'
    }
    // check if all of this supplier's shipments are settled
    const supplierShipments = shipments.value.filter(s => s.supplier_code === shipment.supplier_code)
    if (supplierShipments.length > 0 && supplierShipments.every(s => s.status === '已结算')) {
      // check overall order
      if (shipments.value.every(s => s.status === '已结算')) {
        order.value.status = '已结算'
      }
    }
    ElMessage.success('结算成功')
  }).catch(() => {})
}

// ------- F. Return Dialog -------
const handleReturn = () => {
  ElMessageBox.confirm(
    '确认退货？将通知供应商。',
    '退货确认',
    { confirmButtonText: '确认退货', cancelButtonText: '取消', type: 'warning' }
  ).then(() => {
    ElMessage.success('已发起退货流程')
  }).catch(() => {})
}

// ------- G. Withdraw Dialog -------
const handleWithdraw = () => {
  ElMessageBox.confirm(
    '确认撤回订单？供应商将收到撤销通知。',
    '撤回订单',
    { confirmButtonText: '确认撤回', cancelButtonText: '取消', type: 'danger' }
  ).then(() => {
    order.value.status = '已撤回'
    ElMessage.success('订单已撤回')
  }).catch(() => {})
}

// ------- Navigation -------
const goBack = () => {
  router.push('/procurement/carton/order-list')
}
</script>

<style scoped>
.order-detail {
  padding: 20px;
}

.breadcrumb {
  margin-bottom: 20px;
}

.section-card {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  font-weight: 600;
  font-size: 16px;
}

.action-bar {
  margin-bottom: 20px;
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
  align-items: center;
}

.material-group {
  margin-bottom: 20px;
}

.material-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 8px;
  width: 100%;
}

.material-info {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.material-name {
  font-weight: 600;
  font-size: 15px;
  color: #303133;
}

.material-code {
  font-family: monospace;
  color: #909399;
  font-size: 13px;
}

.material-spec {
  color: #606266;
  font-size: 13px;
}

.material-meta {
  display: flex;
  align-items: center;
  gap: 14px;
  flex-wrap: wrap;
  font-size: 13px;
  color: #606266;
}

.meta-item b {
  color: #303133;
}

.oa-price {
  color: #e6a23c;
}

.supplier-cell {
  display: flex;
  flex-direction: column;
  line-height: 1.4;
}

.supplier-code {
  font-family: monospace;
  font-size: 12px;
  color: #909399;
}

.supplier-name {
  font-weight: 500;
  color: #303133;
}

.text-danger {
  color: #f56c6c;
  font-weight: 600;
}

.text-success {
  color: #67c23a;
  font-weight: 600;
}

.text-amount {
  font-size: 16px;
  font-weight: 700;
  color: #e6a23c;
}

.ship-summary {
  font-size: 14px;
  padding: 8px 0;
  color: #606266;
}

.ship-arrival {
  margin-top: 16px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.ship-arrival .label {
  font-size: 14px;
  color: #606266;
  white-space: nowrap;
}

.code-text {
  color: var(--el-color-primary);
  font-family: monospace;
  font-weight: 500;
}

.ml-8 {
  margin-left: 8px;
}
</style>
