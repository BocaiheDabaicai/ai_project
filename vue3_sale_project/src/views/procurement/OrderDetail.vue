<script setup>
import { ref, reactive, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage, ElMessageBox } from 'element-plus'

const route = useRoute()
const router = useRouter()
const orderId = computed(() => route.params.id)

// ===== 订单基本信息 =====
const order = reactive({
  id: 1,
  orderNo: 'PO20260510001',
  reqNo: 'REQ20260510008',
  name: '生产物料紧急采购',
  status: '已下单',
  totalQty: 2800,
  totalDelivered: 800,
  totalAmount: 89200,
  createTime: '2026-05-10 10:30',
})

// ===== 每项物料 + 供应商分配 =====
const orderItems = ref([
  {
    id: 1, materialCode: 'MAT-001', materialName: '不锈钢板材 304/2mm', spec: '2mm×1500mm×3000mm',
    totalQty: 500, unit: 'kg', price: 28.50,
    suppliers: [
      { code: 'SUP001', name: '浙江优质供应商有限公司', qty: 300, delivered: 200, remain: 100, unitPrice: 28.50, pkg: '标包A' },
      { code: 'SUP002', name: '广州精工制造有限公司', qty: 200, delivered: 100, remain: 100, unitPrice: 27.80, pkg: '标包A' },
    ],
  },
  {
    id: 2, materialCode: 'MAT-002', materialName: '高强度螺栓 M12x50', spec: 'M12×50mm 8.8级',
    totalQty: 2000, unit: 'pcs', price: 0.88,
    suppliers: [
      { code: 'SUP004', name: '北京新材料科技有限公司', qty: 2000, delivered: 500, remain: 1500, unitPrice: 0.88, pkg: '标包B' },
    ],
  },
  {
    id: 3, materialCode: 'MAT-003', materialName: '铜芯电缆 3x2.5mm²', spec: '3芯×2.5mm²',
    totalQty: 300, unit: '卷', price: 152.00,
    suppliers: [
      { code: 'SUP003', name: '上海电子配件有限公司', qty: 300, delivered: 0, remain: 300, unitPrice: 152.00, pkg: '标包C' },
    ],
  },
])

// ===== 操作状态 =====
const canShip = computed(() => order.status === '已下单')
const canReturn = computed(() => order.status === '已发货')
const canWithdraw = computed(() => order.status === '已下单')
const isCompleted = computed(() => order.status === '已结算')

// ===== 发货弹窗 =====
const shipDialog = ref(false)
const shipItems = ref([])

function openShipDialog() {
  shipItems.value = orderItems.value.flatMap(item =>
    item.suppliers.map(s => ({
      materialCode: item.materialCode,
      materialName: item.materialName,
      unit: item.unit,
      supplierCode: s.code,
      supplierName: s.name,
      totalQty: s.qty,
      delivered: s.delivered,
      remain: s.remain,
      shipQty: 0,
    }))
  )
  shipDialog.value = true
}

function submitShip() {
  const toShip = shipItems.value.filter(s => s.shipQty > 0)
  if (toShip.length === 0) {
    ElMessage.warning('请至少分配一项发货数量')
    return
  }
  let totalShipped = 0
  toShip.forEach(s => {
    const supplier = orderItems.value.flatMap(i => i.suppliers).find(sp => sp.code === s.supplierCode && sp.name === s.supplierName)
    if (supplier) {
      supplier.delivered += s.shipQty
      supplier.remain = supplier.qty - supplier.delivered
      totalShipped += s.shipQty
    }
  })
  order.totalDelivered += totalShipped
  const allDone = orderItems.value.flatMap(i => i.suppliers).every(s => s.remain === 0)
  if (allDone) order.status = '已发货'

  // 生成采购子单
  const subOrderNos = toShip.map(s => `SUB-${order.orderNo}-${s.supplierCode}-${Date.now().toString(36).toUpperCase()}`)
  toShip.forEach((s, idx) => {
    subOrders.value.push({
      id: subOrders.value.length + 1,
      subNo: subOrderNos[idx],
      supplierName: s.supplierName,
      supplierCode: s.supplierCode,
      materialName: s.materialName,
      qty: s.shipQty,
      unit: s.unit,
      cost: s.shipQty * (orderItems.value.flatMap(i => i.suppliers).find(sp => sp.code === s.supplierCode)?.unitPrice || 0),
      status: '已发货',
      time: new Date().toLocaleString('zh-CN'),
    })
  })

  ElMessage.success(`已生成 ${toShip.length} 个采购子单，已推送消息给供应商`)
  shipDialog.value = false
}

// ===== 采购子单 =====
const subOrders = ref([
  { id: 1, subNo: 'SUB-PO20260510001-SUP001-001', supplierName: '浙江优质供应商有限公司', supplierCode: 'SUP001', materialName: '不锈钢板材 304/2mm', qty: 200, unit: 'kg', cost: 5700, status: '已发货', time: '2026-05-10 11:00' },
  { id: 2, subNo: 'SUB-PO20260510001-SUP002-001', supplierName: '广州精工制造有限公司', supplierCode: 'SUP002', materialName: '不锈钢板材 304/2mm', qty: 100, unit: 'kg', cost: 2780, status: '已发货', time: '2026-05-10 11:30' },
  { id: 3, subNo: 'SUB-PO20260510001-SUP004-001', supplierName: '北京新材料科技有限公司', supplierCode: 'SUP004', materialName: '高强度螺栓 M12x50', qty: 500, unit: 'pcs', cost: 440, status: '已发货', time: '2026-05-11 09:00' },
])

const subDialog = ref(false)
const subFilterSupplier = ref('')
const currentSubDetail = ref(null)
const subDetailDialog = ref(false)

const filteredSubOrders = computed(() => {
  if (!subFilterSupplier.value) return subOrders.value
  return subOrders.value.filter(s => s.supplierCode === subFilterSupplier.value)
})

function viewSubDetail(sub) {
  currentSubDetail.value = sub
  subDetailDialog.value = true
}

function cancelSubOrder(sub) {
  ElMessageBox.confirm(`确认撤销采购子单 ${sub.subNo}？此操作将通知供应商。`, '撤销确认', { type: 'warning' })
    .then(() => {
      sub.status = '已撤销'
      ElMessage.success(`采购子单 ${sub.subNo} 已撤销`)
    })
    .catch(() => {})
}

// ===== 操作按钮 =====
function handleShip() {
  openShipDialog()
}

function handleReturn() {
  ElMessageBox.confirm('确认退货？系统将生成退货单并通知供应商。', '退货确认', { type: 'warning' })
    .then(() => {
      order.status = '已下单'
      ElMessage.success('已发起退货流程')
    })
    .catch(() => {})
}

function handleWithdraw() {
  ElMessageBox.confirm('确认撤回该采购订单？撤回后供应商将收到撤销通知。', '撤回确认', { type: 'warning' })
    .then(() => {
      ElMessage.success('采购订单已撤回')
      router.back()
    })
    .catch(() => {})
}

function handleComplete() {
  ElMessageBox.confirm('确认该订单已结算完成？', '结算确认', { type: 'info' })
    .then(() => {
      order.status = '已结算'
      ElMessage.success('订单已标记为已结算')
    })
    .catch(() => {})
}
</script>

<template>
  <div class="page-container">
    <!-- 顶部导航 -->
    <div style="display:flex;align-items:center;gap:12px;margin-bottom:20px">
      <el-button :icon="ArrowLeft" @click="router.back()">返回</el-button>
      <h2 class="page-title" style="margin-bottom:0">
        <el-icon :size="20" color="var(--color-accent)"><Document /></el-icon>
        采购订单详情
      </h2>
    </div>

    <!-- 订单基本信息 -->
    <el-descriptions :column="3" border size="small" style="margin-bottom:20px">
      <el-descriptions-item label="订单编号"><span class="code-text">{{ order.orderNo }}</span></el-descriptions-item>
      <el-descriptions-item label="来源请购单">{{ order.reqNo }}</el-descriptions-item>
      <el-descriptions-item label="订单名称">{{ order.name }}</el-descriptions-item>
      <el-descriptions-item label="总采购数量">{{ order.totalQty.toLocaleString() }}</el-descriptions-item>
      <el-descriptions-item label="已发货数量">{{ order.totalDelivered.toLocaleString() }}</el-descriptions-item>
      <el-descriptions-item label="剩余发货数量">{{ (order.totalQty - order.totalDelivered).toLocaleString() }}</el-descriptions-item>
      <el-descriptions-item label="状态">
        <el-tag :type="{ '已下单': 'warning', '已发货': '', '已收货': 'success', '已结算': 'info' }[order.status] || 'info'" effect="light" size="small" round>{{ order.status }}</el-tag>
      </el-descriptions-item>
      <el-descriptions-item label="创建时间">{{ order.createTime }}</el-descriptions-item>
      <el-descriptions-item label="预估金额">¥{{ order.totalAmount.toLocaleString() }}</el-descriptions-item>
    </el-descriptions>

    <!-- 操作按钮 -->
    <div class="action-bar">
      <el-button v-if="canShip" type="primary" :icon="Promotion" @click="handleShip">发货</el-button>
      <el-button v-if="canReturn" type="warning" :icon="RefreshLeft" @click="handleReturn">退货</el-button>
      <el-button v-if="canWithdraw" type="danger" :icon="Close" @click="handleWithdraw">撤回订单</el-button>
      <el-button v-if="order.status === '已收货'" type="success" :icon="Check" @click="handleComplete">确认结算</el-button>
      <el-button :icon="ArrowLeft" @click="router.back()">返回</el-button>
    </div>

    <!-- 物料采购明细 -->
    <h3 style="margin:24px 0 12px;font-size:15px">物料采购明细</h3>
    <div v-for="item in orderItems" :key="item.id" style="margin-bottom:16px;padding:16px;background:var(--bg-page);border-radius:var(--radius)">
      <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:12px">
        <div>
          <strong>{{ item.materialName }}</strong>
          <span style="margin-left:12px;font-size:12px;color:var(--text-secondary)">{{ item.spec }}</span>
        </div>
        <div style="font-size:13px">
          总计：<strong>{{ item.totalQty }}</strong> {{ item.unit }} |
          单价：<strong>¥{{ item.price }}</strong> |
          合计：<strong>¥{{ (item.totalQty * item.price).toLocaleString() }}</strong>
        </div>
      </div>
      <!-- 供应商分配明细 -->
      <div class="table-wrapper">
        <table class="data-table" style="font-size:12px">
          <thead>
            <tr>
              <th>供应商</th><th>标包</th><th>分配数量</th><th>已发货</th><th>剩余</th><th>单价</th><th>小计</th><th>采购子单</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="s in item.suppliers" :key="s.code">
              <td>{{ s.name }}<el-tag size="small" style="margin-left:4px">{{ s.code }}</el-tag></td>
              <td><el-tag effect="plain" size="small" type="warning">{{ s.pkg }}</el-tag></td>
              <td>{{ s.qty }} {{ item.unit }}</td>
              <td>{{ s.delivered }} {{ item.unit }}</td>
              <td><span :style="{ color: s.remain > 0 ? '#f56c6c' : '#67c23a', fontWeight: 600 }">{{ s.remain }} {{ item.unit }}</span></td>
              <td>¥{{ s.unitPrice }}</td>
              <td>¥{{ (s.qty * s.unitPrice).toLocaleString() }}</td>
              <td>
                <el-button type="primary" link size="small" @click="subFilterSupplier = s.code; subDialog = true">追溯查询</el-button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- ===== 发货弹窗 ===== -->
    <el-dialog v-model="shipDialog" title="发货管理" width="800px" top="5vh">
      <div style="margin-bottom:12px;font-size:13px;color:var(--text-secondary)">
        订单总采购数量：<strong>{{ order.totalQty.toLocaleString() }}</strong> |
        已发货：<strong>{{ order.totalDelivered.toLocaleString() }}</strong> |
        剩余：<strong style="color:#f56c6c">{{ (order.totalQty - order.totalDelivered).toLocaleString() }}</strong>
      </div>
      <div class="table-wrapper">
        <table class="data-table">
          <thead>
            <tr><th>物料</th><th>供应商</th><th>总分配</th><th>已发货</th><th>剩余</th><th>本次发货数量</th></tr>
          </thead>
          <tbody>
            <tr v-for="(s, idx) in shipItems" :key="idx">
              <td>{{ s.materialName }}</td>
              <td>{{ s.supplierName }}</td>
              <td>{{ s.totalQty }} {{ s.unit }}</td>
              <td>{{ s.delivered }} {{ s.unit }}</td>
              <td><strong :style="{ color: s.remain > 0 ? '#f56c6c' : '#67c23a' }">{{ s.remain }} {{ s.unit }}</strong></td>
              <td>
                <el-input-number
                  v-model="s.shipQty"
                  :min="0"
                  :max="s.remain"
                  size="small"
                  style="width:130px"
                  :disabled="s.remain <= 0"
                />
                <span style="margin-left:4px;font-size:12px">{{ s.unit }}</span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <template #footer>
        <el-button @click="shipDialog = false">取消</el-button>
        <el-button type="primary" @click="submitShip">确认发货并生成子单</el-button>
      </template>
    </el-dialog>

    <!-- ===== 采购子单追溯弹窗 ===== -->
    <el-dialog v-model="subDialog" title="采购子单追溯" width="900px" top="5vh">
      <div class="table-wrapper">
        <table class="data-table">
          <thead>
            <tr><th>子单编号</th><th>供应商</th><th>物料</th><th>数量</th><th>费用</th><th>状态</th><th>时间</th><th>操作</th></tr>
          </thead>
          <tbody>
            <tr v-for="sub in filteredSubOrders" :key="sub.id">
              <td><span class="code-text">{{ sub.subNo }}</span></td>
              <td>{{ sub.supplierName }}</td>
              <td>{{ sub.materialName }}</td>
              <td>{{ sub.qty }} {{ sub.unit }}</td>
              <td>¥{{ sub.cost.toLocaleString() }}</td>
              <td>
                <el-tag :type="sub.status === '已发货' ? 'success' : sub.status === '已撤销' ? 'danger' : 'info'" effect="light" size="small" round>{{ sub.status }}</el-tag>
              </td>
              <td>{{ sub.time }}</td>
              <td>
                <el-button type="primary" link size="small" @click="viewSubDetail(sub)">详情</el-button>
                <el-button v-if="sub.status === '已发货'" type="danger" link size="small" @click="cancelSubOrder(sub)">撤销</el-button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <template #footer>
        <el-button @click="subDialog = false">关闭</el-button>
      </template>
    </el-dialog>

    <!-- ===== 子单详情弹窗 ===== -->
    <el-dialog v-model="subDetailDialog" title="采购子单详情" width="500px" top="10vh">
      <template v-if="currentSubDetail">
        <el-descriptions :column="2" border size="small">
          <el-descriptions-item label="子单编号">{{ currentSubDetail.subNo }}</el-descriptions-item>
          <el-descriptions-item label="供应商">{{ currentSubDetail.supplierName }}</el-descriptions-item>
          <el-descriptions-item label="物料">{{ currentSubDetail.materialName }}</el-descriptions-item>
          <el-descriptions-item label="采购数量">{{ currentSubDetail.qty }} {{ currentSubDetail.unit }}</el-descriptions-item>
          <el-descriptions-item label="产生费用">¥{{ currentSubDetail.cost.toLocaleString() }}</el-descriptions-item>
          <el-descriptions-item label="状态">{{ currentSubDetail.status }}</el-descriptions-item>
          <el-descriptions-item label="创建时间" :span="2">{{ currentSubDetail.time }}</el-descriptions-item>
        </el-descriptions>
      </template>
      <template #footer>
        <el-button @click="subDetailDialog = false">关闭</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.code-text {
  font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  color: var(--color-accent);
}
.action-bar {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}
</style>
