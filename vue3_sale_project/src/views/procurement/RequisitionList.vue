<script setup>
import { ref, reactive, computed } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useRouter } from 'vue-router'

const router = useRouter()

// ========== 请购单列表 ==========
const searchReqNo = ref('')
const searchStatus = ref('')

const requisitions = ref([
  { id: 1, reqNo: 'REQ20260510008', dept: '生产部', applicant: '赵工', itemCount: 12, urgency: '紧急', status: '待处理', pushTime: '2026-05-10 09:30' },
  { id: 2, reqNo: 'REQ20260509007', dept: '研发部', applicant: '钱工', itemCount: 5, urgency: '普通', status: '处理中', pushTime: '2026-05-09 14:20' },
  { id: 3, reqNo: 'REQ20260509006', dept: '生产部', applicant: '孙工', itemCount: 8, urgency: '紧急', status: '待处理', pushTime: '2026-05-09 10:00' },
  { id: 4, reqNo: 'REQ20260508005', dept: '设备部', applicant: '李工', itemCount: 3, urgency: '普通', status: '已处理', pushTime: '2026-05-08 16:45' },
  { id: 5, reqNo: 'REQ20260507004', dept: '品质部', applicant: '周工', itemCount: 15, urgency: '普通', status: '已处理', pushTime: '2026-05-07 08:30' },
])

// ========== 请购单详情预览 ==========
const detailDrawer = ref(false)
const currentReq = ref(null)
const reqMaterials = ref([
  { id: 1, code: 'MAT-001', name: '不锈钢板材 304/2mm', spec: '2mm×1500mm×3000mm', qty: 500, unit: 'kg', category: '金属材料' },
  { id: 2, code: 'MAT-002', name: '高强度螺栓 M12x50', spec: 'M12×50mm 8.8级', qty: 2000, unit: 'pcs', category: '紧固件' },
  { id: 3, code: 'MAT-003', name: '铜芯电缆 3x2.5mm²', spec: '3芯×2.5mm²', qty: 300, unit: '卷', category: '电气材料' },
])

// ========== 供应商报价 ==========
const suppliers = ref([
  { id: 1, name: '浙江优质供应商有限公司', code: 'SUP001' },
  { id: 2, name: '广州精工制造有限公司', code: 'SUP002' },
  { id: 3, name: '上海电子配件有限公司', code: 'SUP003' },
  { id: 4, name: '北京新材料科技有限公司', code: 'SUP004' },
])

// 每个物料在每个供应商处的报价，null表示无报价
const priceMatrix = reactive({
  'MAT-001': {
    'SUP001': { price: 28.50, pkg: '标包A' },
    'SUP002': { price: 27.80, pkg: '标包A' },
    'SUP003': { price: 29.20, pkg: '标包A' },
    'SUP004': null,
  },
  'MAT-002': {
    'SUP001': { price: 0.85, pkg: '标包B' },
    'SUP002': { price: 0.92, pkg: '标包B' },
    'SUP003': null,
    'SUP004': { price: 0.88, pkg: '标包B' },
  },
  'MAT-003': {
    'SUP001': null,
    'SUP002': { price: 156.00, pkg: '标包C' },
    'SUP003': { price: 152.00, pkg: '标包C' },
    'SUP004': { price: 160.00, pkg: '标包C' },
  },
})

// ========== 创建采购订单流程（弹窗） ==========
const createDialog = ref(false)
const step = ref(0)
const selectedReq = ref(null)

// Step 1: 物料勾选
const materialSelection = ref([])
const allChecked = computed({
  get: () => materialSelection.value.length > 0 && materialSelection.value.every(m => m.checked),
  set: (v) => materialSelection.value.forEach(m => m.checked = v),
})

// Step 2: 比价确价
const confirmedPrices = reactive({})

// Step 3: 分配数量
const allocations = reactive({})

// Step 4: 确认生成
const finalOrder = ref(null)

function openCreatePO(req) {
  selectedReq.value = req
  step.value = 0
  createDialog.value = true
  // 初始化：默认全部勾选
  materialSelection.value = reqMaterials.value.map(m => ({ ...m, checked: true }))
  // 清空确价和分配
  Object.keys(confirmedPrices).forEach(k => delete confirmedPrices[k])
  Object.keys(allocations).forEach(k => delete allocations[k])
  // 初始化默认价格和分配
  reqMaterials.value.forEach(m => {
    const prices = priceMatrix[m.code] || {}
    const lowest = Object.entries(prices)
      .filter(([, v]) => v !== null)
      .sort(([, a], [, b]) => a.price - b.price)[0]
    if (lowest) {
      const [supCode, info] = lowest
      confirmedPrices[m.code] = { supplierCode: supCode, supplierName: suppliers.value.find(s => s.code === supCode)?.name, price: info.price, pkg: info.pkg }
    }
    allocations[m.code] = []
  })
}

function hasMissingPrice(materialCode) {
  const prices = priceMatrix[materialCode] || {}
  return Object.values(prices).some(v => v === null)
}

function getSupplierPrices(materialCode) {
  return Object.entries(priceMatrix[materialCode] || {}).map(([code, info]) => {
    const sup = suppliers.value.find(s => s.code === code)
    return { supplierCode: code, supplierName: sup?.name, price: info?.price, pkg: info?.pkg }
  })
}

function selectPrice(materialCode, supplierCode, supplierName, price, pkg) {
  confirmedPrices[materialCode] = { supplierCode, supplierName, price, pkg }
  allocations[materialCode] = []
}

function addAllocation(materialCode) {
  if (!allocations[materialCode]) allocations[materialCode] = []
  allocations[materialCode].push({ supplierCode: '', supplierName: '', qty: 0 })
}

function removeAllocation(materialCode, idx) {
  allocations[materialCode].splice(idx, 1)
}

function nextStep() {
  if (step.value === 0) {
    const checked = materialSelection.value.filter(m => m.checked)
    if (checked.length === 0) {
      ElMessage.warning('请至少选择一项物料')
      return
    }
  }
  if (step.value === 1) {
    const unchecked = materialSelection.value.filter(m => m.checked).some(m => !confirmedPrices[m.code])
    if (unchecked) {
      ElMessage.warning('请为每一项物料确认采购价格')
      return
    }
  }
  if (step.value === 2) {
    let totalAllocated = 0
    materialSelection.value.filter(m => m.checked).forEach(m => {
      const list = allocations[m.code] || []
      totalAllocated += list.reduce((s, a) => s + (Number(a.qty) || 0), 0)
    })
    if (totalAllocated === 0) {
      ElMessage.warning('请为物料分配采购数量')
      return
    }
  }
  step.value++
  if (step.value === 3) {
    buildFinalOrder()
  }
}

function prevStep() {
  step.value--
}

function buildFinalOrder() {
  const items = materialSelection.value.filter(m => m.checked).map(m => {
    const price = confirmedPrices[m.code]
    const allocated = (allocations[m.code] || []).filter(a => a.supplierCode && a.qty > 0)
    return {
      materialCode: m.code,
      materialName: m.name,
      spec: m.spec,
      totalQty: m.qty,
      unit: m.unit,
      selectedSupplier: price?.supplierName,
      selectedPrice: price?.price,
      pkg: price?.pkg,
      supplierAllocations: allocated,
    }
  })
  finalOrder.value = {
    reqNo: selectedReq.value?.reqNo,
    items,
    totalAmount: items.reduce((s, i) => s + (i.selectedPrice || 0) * i.totalQty, 0),
    createTime: new Date().toLocaleString('zh-CN'),
  }
}

function submitOrder() {
  ElMessage.success(`采购订单已生成，已推送消息给供应商`)
  createDialog.value = false
  selectedReq.value.status = '已处理'
}

const stepTitles = ['选择采购物料', '比价与确价', '分配采购数量', '确认并生成订单']
</script>

<template>
  <div class="page-container">
    <h2 class="page-title">
      <el-icon :size="20" color="var(--color-accent)"><Tickets /></el-icon>
      请购单管理
    </h2>

    <!-- 搜索栏 -->
    <div class="search-bar">
      <el-input v-model="searchReqNo" placeholder="请购单编号" clearable style="width:180px" />
      <el-select v-model="searchStatus" placeholder="全部状态" clearable style="width:130px">
        <el-option label="待处理" value="待处理" />
        <el-option label="处理中" value="处理中" />
        <el-option label="已处理" value="已处理" />
      </el-select>
      <el-button type="primary" :icon="Search">查询</el-button>
      <el-button :icon="RefreshRight">重置</el-button>
    </div>

    <!-- 请购单表格 -->
    <div class="table-wrapper">
      <table class="data-table">
        <thead>
          <tr>
            <th>#</th>
            <th>请购单编号</th>
            <th>申请部门</th>
            <th>申请人</th>
            <th>物料数量</th>
            <th>紧急程度</th>
            <th>状态</th>
            <th>推送时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(item, idx) in requisitions" :key="item.id">
            <td>{{ idx + 1 }}</td>
            <td><span class="code-text">{{ item.reqNo }}</span></td>
            <td>{{ item.dept }}</td>
            <td>{{ item.applicant }}</td>
            <td>{{ item.itemCount }} 项</td>
            <td>
              <el-tag :type="item.urgency === '紧急' ? 'danger' : ''" effect="light" size="small" round>
                {{ item.urgency }}
              </el-tag>
            </td>
            <td>
              <el-tag :type="item.status === '待处理' ? 'warning' : item.status === '已处理' ? 'success' : ''" effect="light" size="small" round>
                {{ item.status }}
              </el-tag>
            </td>
            <td>{{ item.pushTime }}</td>
            <td>
              <el-button type="primary" link size="small" @click="detailDrawer = true; currentReq = item">预览详情</el-button>
              <el-button
                v-if="item.status === '待处理'"
                type="success"
                link
                size="small"
                @click="openCreatePO(item)"
              >创建采购订单</el-button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- ===== 请购单详情抽屉 ===== -->
    <el-drawer v-model="detailDrawer" title="请购单详情" size="500px">
      <template v-if="currentReq">
        <el-descriptions :column="2" border size="small" style="margin-bottom:16px">
          <el-descriptions-item label="请购单编号">{{ currentReq.reqNo }}</el-descriptions-item>
          <el-descriptions-item label="申请部门">{{ currentReq.dept }}</el-descriptions-item>
          <el-descriptions-item label="申请人">{{ currentReq.applicant }}</el-descriptions-item>
          <el-descriptions-item label="紧急程度">{{ currentReq.urgency }}</el-descriptions-item>
          <el-descriptions-item label="状态">{{ currentReq.status }}</el-descriptions-item>
          <el-descriptions-item label="推送时间">{{ currentReq.pushTime }}</el-descriptions-item>
        </el-descriptions>
        <h4 style="margin-bottom:12px">物料明细</h4>
        <div class="table-wrapper">
          <table class="data-table">
            <thead>
              <tr><th>编码</th><th>名称</th><th>规格</th><th>数量</th><th>单位</th></tr>
            </thead>
            <tbody>
              <tr v-for="m in reqMaterials" :key="m.id">
                <td>{{ m.code }}</td><td>{{ m.name }}</td><td>{{ m.spec }}</td><td>{{ m.qty }}</td><td>{{ m.unit }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </template>
    </el-drawer>

    <!-- ===== 创建采购订单弹窗（多步流程） ===== -->
    <el-dialog
      v-model="createDialog"
      :title="'创建采购订单 - ' + selectedReq?.reqNo"
      width="900px"
      :close-on-click-modal="false"
      top="5vh"
    >
      <!-- 步骤条 -->
      <el-steps :active="step" align-center finish-status="success" style="margin-bottom:24px">
        <el-step v-for="(t, i) in stepTitles" :key="i" :title="t" />
      </el-steps>

      <!-- ===== Step 0: 选择物料 ===== -->
      <div v-if="step === 0">
        <p style="margin-bottom:12px;color:var(--text-secondary);font-size:13px">
          以下为请购单中的全部物料，已自动勾选您负责采购的物料。
        </p>
        <div class="table-wrapper">
          <table class="data-table">
            <thead>
              <tr><th><el-checkbox v-model="allChecked" /></th><th>物料编码</th><th>物料名称</th><th>规格</th><th>请购数量</th><th>单位</th><th>类别</th></tr>
            </thead>
            <tbody>
              <tr v-for="m in materialSelection" :key="m.id">
                <td><el-checkbox v-model="m.checked" /></td>
                <td><span class="code-text">{{ m.code }}</span></td>
                <td>{{ m.name }}</td>
                <td>{{ m.spec }}</td>
                <td>{{ m.qty }}</td>
                <td>{{ m.unit }}</td>
                <td>{{ m.category }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- ===== Step 1: 比价确价 ===== -->
      <div v-if="step === 1">
        <p style="margin-bottom:12px;color:var(--text-secondary);font-size:13px">
          为每项物料选择供应商和价格，标注了<el-tag type="danger" size="small">缺少报价</el-tag>的物料需要人工比价。
        </p>
        <div v-for="m in materialSelection.filter(m => m.checked)" :key="m.id" style="margin-bottom:20px;padding:16px;background:var(--bg-page);border-radius:var(--radius)">
          <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:10px">
            <strong>{{ m.name }}</strong>
            <span style="font-size:12px;color:var(--text-secondary)">{{ m.code }} · {{ m.spec }} · {{ m.qty }}{{ m.unit }}</span>
            <el-tag v-if="hasMissingPrice(m.code)" type="danger" size="small">缺少报价，需比价</el-tag>
            <el-tag v-else type="success" size="small">报价齐全</el-tag>
          </div>
          <div class="table-wrapper">
            <table class="data-table">
              <thead>
                <tr><th>供应商</th><th>报价(元)</th><th>标包出处</th><th>操作</th></tr>
              </thead>
              <tbody>
                <tr v-for="sp in getSupplierPrices(m.code)" :key="sp.supplierCode">
                  <td>{{ sp.supplierName }}<el-tag size="small" style="margin-left:6px">{{ sp.supplierCode }}</el-tag></td>
                  <td>
                    <span v-if="sp.price !== undefined && sp.price !== null" :style="{ fontWeight: 600 }">¥{{ sp.price }}</span>
                    <el-tag v-else type="danger" size="small">暂无报价</el-tag>
                  </td>
                  <td>
                    <el-tag v-if="sp.pkg" effect="plain" size="small" type="warning">{{ sp.pkg }}</el-tag>
                    <span v-else style="color:var(--text-muted)">—</span>
                  </td>
                  <td>
                    <el-button
                      v-if="sp.price !== undefined && sp.price !== null"
                      :type="confirmedPrices[m.code]?.supplierCode === sp.supplierCode ? 'primary' : 'default'"
                      size="small"
                      @click="selectPrice(m.code, sp.supplierCode, sp.supplierName, sp.price, sp.pkg)"
                    >{{ confirmedPrices[m.code]?.supplierCode === sp.supplierCode ? '已选择 ✓' : '选择' }}</el-button>
                    <el-button v-else size="small" disabled>无法选择</el-button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
          <div v-if="confirmedPrices[m.code]" style="margin-top:8px;font-size:13px;color:var(--color-accent)">
            已确认：{{ confirmedPrices[m.code].supplierName }} —
            ¥{{ confirmedPrices[m.code].price }}/{{ m.unit }}
            <el-tag effect="plain" size="small" type="warning" style="margin-left:4px">{{ confirmedPrices[m.code].pkg }}</el-tag>
          </div>
        </div>
      </div>

      <!-- ===== Step 2: 分配数量 ===== -->
      <div v-if="step === 2">
        <p style="margin-bottom:12px;color:var(--text-secondary);font-size:13px">
          根据标包为各供应商分配采购数量。标包相同的物料应尽量分配给同一供应商。
        </p>
        <div v-for="m in materialSelection.filter(m => m.checked)" :key="m.id" style="margin-bottom:20px;padding:16px;background:var(--bg-page);border-radius:var(--radius)">
          <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:10px">
            <strong>{{ m.name }}</strong>
            <span>
              需采购 <span style="color:var(--color-accent);font-size:16px">{{ m.qty }}</span> {{ m.unit }}
              &nbsp;|&nbsp;
              已分配 <span :style="{ color: (allocations[m.code]||[]).reduce((s,a) => s+(Number(a.qty)||0), 0) === m.qty ? '#67c23a' : '#f56c6c', fontSize:'16px' }">{{ (allocations[m.code] || []).reduce((s, a) => s + (Number(a.qty) || 0), 0) }}</span> {{ m.unit }}
              <el-tag v-if="confirmedPrices[m.code]" effect="plain" size="small" type="warning" style="margin-left:6px">{{ confirmedPrices[m.code].pkg }}</el-tag>
            </span>
          </div>
          <div v-for="(alloc, ai) in (allocations[m.code] || [])" :key="ai" style="display:flex;gap:10px;align-items:center;margin-bottom:6px">
            <span style="font-size:12px;color:var(--text-secondary);width:24px">#{{ ai + 1 }}</span>
            <el-select v-model="alloc.supplierCode" placeholder="选择供应商" size="small" style="width:220px" @change="(v) => { const s = suppliers.find(s => s.code === v); alloc.supplierName = s?.name }">
              <el-option v-for="s in suppliers" :key="s.code" :label="s.name" :value="s.code" />
            </el-select>
            <el-input-number v-model="alloc.qty" :min="0" :max="m.qty" size="small" :placeholder="'数量(' + m.unit + ')'" style="width:160px" />
            <el-button :icon="Delete" circle size="small" type="danger" @click="removeAllocation(m.code, ai)" />
          </div>
          <el-button size="small" :icon="Plus" @click="addAllocation(m.code)">添加供应商分配</el-button>
        </div>
      </div>

      <!-- ===== Step 3: 确认生成 ===== -->
      <div v-if="step === 3">
        <el-alert title="确认采购订单信息" type="success" :closable="false" style="margin-bottom:16px" />
        <div class="table-wrapper">
          <table class="data-table">
            <thead>
              <tr><th>物料</th><th>规格</th><th>总数量</th><th>供应商</th><th>单价(元)</th><th>标包</th><th>分配明细</th></tr>
            </thead>
            <tbody>
              <tr v-for="item in finalOrder?.items" :key="item.materialCode">
                <td>{{ item.materialName }}</td>
                <td>{{ item.spec }}</td>
                <td>{{ item.totalQty }} {{ item.unit }}</td>
                <td>{{ item.selectedSupplier }}</td>
                <td>¥{{ item.selectedPrice }}</td>
                <td><el-tag effect="plain" size="small" type="warning">{{ item.pkg }}</el-tag></td>
                <td>
                  <span v-for="a in item.supplierAllocations" :key="a.supplierCode" style="display:block;font-size:12px">
                    {{ a.supplierName }}: {{ a.qty }} {{ item.unit }}
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
        <div style="text-align:right;margin-top:16px;font-size:16px">
          预估总金额：<span style="color:var(--color-accent);font-weight:700">¥{{ finalOrder?.totalAmount?.toLocaleString() }}</span>
        </div>
        <el-divider />
        <p style="font-size:13px;color:var(--text-secondary)">生成订单后将自动推送消息给对应的供应商。</p>
      </div>

      <template #footer>
        <div style="display:flex;justify-content:space-between">
          <div>
            <el-button v-if="step > 0" @click="prevStep">上一步</el-button>
          </div>
          <div>
            <el-button @click="createDialog = false">取消</el-button>
            <el-button v-if="step < 3" type="primary" @click="nextStep">下一步</el-button>
            <el-button v-if="step === 3" type="success" @click="submitOrder">
              生成采购订单
            </el-button>
          </div>
        </div>
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
</style>
