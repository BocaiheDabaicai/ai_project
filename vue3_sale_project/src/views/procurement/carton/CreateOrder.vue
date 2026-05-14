<template>
  <div class="procurement-container">
    <!-- Page Header -->
    <div class="page-header">
      <h1 class="page-title">创建纸箱采购订单</h1>
      <p class="page-subtitle">申请单号：REQ{{ reqId }}</p>
    </div>

    <!-- Steps Indicator -->
    <el-card shadow="never" class="steps-card">
      <el-steps :active="activeStep" align-center finish-status="success">
        <el-step title="策略与价格分配" />
        <el-step title="选择供应商" />
        <el-step title="卡纸配置" />
        <el-step title="分配供应数量" />
        <el-step title="确认提交" />
      </el-steps>
    </el-card>

    <!-- ============================================================ -->
    <!-- Step 1: 策略与价格分配                                         -->
    <!-- ============================================================ -->
    <div v-if="activeStep === 0" class="step-content">
      <div v-for="mat in materials" :key="mat.code" class="material-section">
        <el-card shadow="never">
          <template #header>
            <div class="material-header">
              <div class="material-title-row">
                <strong>{{ mat.code }}</strong>
                <span class="material-name">{{ mat.name }}</span>
                <el-tag v-if="mat.isNew" type="danger" size="small" effect="dark">新品</el-tag>
              </div>
            </div>
          </template>

          <!-- Material Basic Info -->
          <el-descriptions :column="4" border size="small">
            <el-descriptions-item label="规格" :span="2">{{ mat.spec }}</el-descriptions-item>
            <el-descriptions-item label="数量">{{ mat.qty.toLocaleString() }}</el-descriptions-item>
            <el-descriptions-item label="单位">{{ mat.unit }}</el-descriptions-item>
          </el-descriptions>

          <!-- Pricing Strategy -->
          <div class="strategy-row">
            <span class="strategy-label">定价策略：</span>
            <el-select
              v-model="pricingStrategy[mat.code]"
              style="width: 140px"
              @change="(val) => onStrategyChange(mat.code, val)"
            >
              <el-option label="最低价" value="最低价" />
              <el-option label="自定义" value="自定义" />
            </el-select>
            <span v-if="pricingStrategy[mat.code] === '最低价'" class="strategy-hint">
              系统将自动选择最低报价
            </span>
            <span v-else class="strategy-hint">
              请手动选择供应商报价
            </span>
          </div>

          <!-- Supplier Quotes Table -->
          <div class="quotes-section">
            <h4 class="quotes-title">2026年Q2供应商报价</h4>
            <el-table :data="getMaterialQuotes(mat.code)" border size="small" style="width: 100%">
              <el-table-column prop="supplierName" label="供应商" min-width="150" />
              <el-table-column label="价格(元/个)" width="130" align="right">
                <template #default="{ row }">
                  <span v-if="row.price !== null" class="price-value">
                    ¥{{ row.price.toFixed(2) }}
                  </span>
                  <el-tag v-else type="info" size="small">暂无报价</el-tag>
                </template>
              </el-table-column>
              <el-table-column prop="pkg" label="标包" width="90" align="center" />
              <el-table-column prop="grade" label="等级" width="80" align="center" />
              <el-table-column label="操作" width="100" align="center">
                <template #default="{ row }">
                  <el-button
                    v-if="row.price !== null"
                    :type="selectedQuoteId[mat.code] === row.id ? 'primary' : 'default'"
                    size="small"
                    :disabled="pricingStrategy[mat.code] === '最低价' && selectedQuoteId[mat.code] !== row.id"
                    @click="selectQuote(mat.code, row)"
                  >
                    {{ selectedQuoteId[mat.code] === row.id ? '已选' : '选择' }}
                  </el-button>
                  <el-tag v-else type="info" size="small" effect="plain">暂无报价</el-tag>
                </template>
              </el-table-column>
            </el-table>

            <!-- No Quotes Notice -->
            <el-alert
              v-if="getMaterialQuotes(mat.code).length === 0"
              title="该物料暂无供应商报价"
              type="warning"
              :closable="false"
              show-icon
              style="margin-top: 8px"
            />
          </div>

          <!-- OA Price for New Products -->
          <div v-if="mat.isNew && mat.oaPrice" class="oa-row">
            <el-tag type="warning" effect="dark" size="small">OA参考价</el-tag>
            <span class="oa-price">¥{{ mat.oaPrice.toFixed(2) }} / {{ mat.unit }}</span>
            <span class="oa-hint">新品以OA价格为基准</span>
          </div>
        </el-card>
      </div>
    </div>

    <!-- ============================================================ -->
    <!-- Step 2: 供应商选择                                             -->
    <!-- ============================================================ -->
    <div v-if="activeStep === 1" class="step-content">
      <div v-for="mat in materials" :key="mat.code" class="material-section">
        <el-card shadow="never">
          <template #header>
            <div class="material-header">
              <div class="material-title-row">
                <strong>{{ mat.code }}</strong>
                <span class="material-name">{{ mat.name }}</span>
                <span class="qty-badge">需求量：{{ mat.qty.toLocaleString() }} {{ mat.unit }}</span>
              </div>
            </div>
          </template>

          <el-row :gutter="16">
            <el-col
              v-for="sup in suppliers"
              :key="sup.id"
              :xs="24"
              :sm="12"
              :md="8"
              :lg="8"
              style="margin-bottom: 16px"
            >
              <el-card
                shadow="hover"
                :class="['supplier-card', { 'is-selected': supplierSelections[mat.code]?.[sup.id]?.selected }]"
              >
                <div class="supplier-card-header">
                  <el-checkbox
                    v-model="supplierSelections[mat.code][sup.id].selected"
                    @change="onSupplierSelectChange(mat.code, sup.id)"
                  >
                    <strong class="supplier-name">{{ sup.name }}</strong>
                  </el-checkbox>
                  <el-tag size="small" type="info" effect="plain">{{ sup.pkg }}</el-tag>
                </div>

                <el-tag v-if="sup.grade === 'A级'" size="small" type="success" effect="light">A级</el-tag>
                <el-tag v-else size="small" type="warning" effect="light">B级</el-tag>

                <div class="quality-fields">
                  <div class="quality-field">
                    <label class="field-label">送测结果</label>
                    <el-select
                      v-model="supplierSelections[mat.code][sup.id].testResult"
                      size="small"
                      style="width: 100%"
                    >
                      <el-option
                        v-for="opt in testResultOptions"
                        :key="opt.value"
                        :label="opt.label"
                        :value="opt.value"
                      />
                    </el-select>
                  </div>

                  <div class="quality-field">
                    <label class="field-label">色差</label>
                    <el-select
                      v-model="supplierSelections[mat.code][sup.id].colorDiff"
                      size="small"
                      style="width: 100%"
                    >
                      <el-option
                        v-for="opt in colorDiffOptions"
                        :key="opt.value"
                        :label="opt.label"
                        :value="opt.value"
                      />
                    </el-select>
                  </div>

                  <div class="quality-field">
                    <label class="field-label">质量</label>
                    <el-select
                      v-model="supplierSelections[mat.code][sup.id].quality"
                      size="small"
                      style="width: 100%"
                    >
                      <el-option
                        v-for="opt in qualityOptions"
                        :key="opt.value"
                        :label="opt.label"
                        :value="opt.value"
                      />
                    </el-select>
                  </div>

                  <div class="quality-field">
                    <label class="field-label">评分</label>
                    <el-rate
                      v-model="supplierSelections[mat.code][sup.id].score"
                      :max="5"
                      :low-threshold="2"
                      :high-threshold="4"
                      :colors="['#f56c6c', '#e6a23c', '#67c23a']"
                      size="small"
                    />
                    <span class="score-value">
                      {{ supplierSelections[mat.code][sup.id].score.toFixed(1) }}
                    </span>
                  </div>
                </div>
              </el-card>
            </el-col>
          </el-row>
        </el-card>
      </div>
    </div>

    <!-- ============================================================ -->
    <!-- Step 3: 卡纸配置                                              -->
    <!-- ============================================================ -->
    <div v-if="activeStep === 2" class="step-content">
      <el-card shadow="never">
        <template #header>
          <div class="material-header">
            <span>卡纸配置</span>
            <el-button size="small" text @click="clearAllCardPaper">
              <el-icon><Close /></el-icon>
              清空全部
            </el-button>
          </div>
        </template>

        <div v-for="mat in materials" :key="mat.code" class="card-paper-row">
          <div class="card-paper-label">
            <strong>{{ mat.code }}</strong>
            <span class="material-name">{{ mat.name }}</span>
          </div>
          <div class="card-paper-input">
            <el-input
              v-model="cardPaperConfig[mat.code]"
              placeholder="请输入卡纸配置信息"
              clearable
            >
              <template #prepend>卡纸</template>
            </el-input>
          </div>
          <div class="card-paper-actions">
            <el-button
              size="small"
              :disabled="!cardPaperConfig[mat.code]"
              @click="copyCardPaper(mat.code)"
            >
              <el-icon><CopyDocument /></el-icon>
              复制
            </el-button>
            <el-button
              size="small"
              :disabled="!clipboardCardPaper"
              @click="pasteCardPaper(mat.code)"
            >
              <el-icon><DocumentCopy /></el-icon>
              粘贴
            </el-button>
          </div>
        </div>

        <!-- Quick Copy All -->
        <el-divider />
        <div class="quick-copy-row">
          <span class="quick-copy-label">批量操作：</span>
          <el-button size="small" @click="copyAllCardPaper">
            <el-icon><CopyDocument /></el-icon>
            复制全部
          </el-button>
          <el-button
            size="small"
            :disabled="!clipboardCardPaper"
            @click="pasteAllCardPaper"
          >
            <el-icon><DocumentCopy /></el-icon>
            粘贴到全部
          </el-button>
        </div>
      </el-card>
    </div>

    <!-- ============================================================ -->
    <!-- Step 4: 分配供应数量                                          -->
    <!-- ============================================================ -->
    <div v-if="activeStep === 3" class="step-content">
      <div v-for="mat in materials" :key="mat.code" class="material-section">
        <el-card shadow="never">
          <template #header>
            <div class="material-header">
              <div class="material-title-row">
                <strong>{{ mat.code }}</strong>
                <span class="material-name">{{ mat.name }}</span>
                <span class="qty-badge">需求：{{ mat.qty.toLocaleString() }} {{ mat.unit }}</span>
              </div>
            </div>
          </template>

          <el-table
            v-if="getAllocationRows(mat.code).length > 0"
            :data="getAllocationRows(mat.code)"
            border
            size="small"
            style="width: 100%"
          >
            <el-table-column type="index" label="#" width="50" align="center" />
            <el-table-column prop="supplierName" label="供应商" min-width="150" />
            <el-table-column prop="pkg" label="标包" width="80" align="center" />
            <el-table-column prop="location" label="生产地" width="100" align="center" />
            <el-table-column label="标准样箱" width="100" align="center">
              <template #default="{ row }">
                <el-tag :type="row.isStandardSample ? 'success' : 'info'" size="small">
                  {{ row.isStandardSample ? '是' : '否' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column label="供过大货" width="100" align="center">
              <template #default="{ row }">
                <el-tag :type="row.hasBulkHistory ? 'success' : 'info'" size="small">
                  {{ row.hasBulkHistory ? '是' : '否' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column label="上次分配量" width="120" align="right">
              <template #default="{ row }">
                {{ row.lastAllocationQty.toLocaleString() }}
              </template>
            </el-table-column>
            <el-table-column label="本次分配数量" width="160" align="center">
              <template #default="{ row }">
                <el-input-number
                  v-model="allocationData[row.materialCode][row.supplierId].qty"
                  :min="0"
                  :max="getRequiredQty(row.materialCode)"
                  :precision="0"
                  size="small"
                  controls-position="right"
                  style="width: 130px"
                />
              </template>
            </el-table-column>
            <el-table-column label="备注" min-width="160">
              <template #default="{ row }">
                <el-input
                  v-model="allocationData[row.materialCode][row.supplierId].remark"
                  placeholder="备注信息"
                  size="small"
                />
              </template>
            </el-table-column>
          </el-table>

          <!-- No Selected Suppliers Warning -->
          <el-alert
            v-if="getAllocationRows(mat.code).length === 0"
            title="该物料未选择供应商，请返回上一步进行选择"
            type="warning"
            :closable="false"
            show-icon
          />

          <!-- Allocation Summary -->
          <div class="allocation-summary">
            <span class="summary-label">分配汇总：</span>
            <span>已分配：<strong>{{ getAllocationTotal(mat.code).toLocaleString() }}</strong></span>
            <span>需求：<strong>{{ mat.qty.toLocaleString() }}</strong></span>
            <el-tag
              :type="getAllocationTotal(mat.code) === mat.qty ? 'success' : 'danger'"
              size="small"
              effect="dark"
            >
              {{ getAllocationTotal(mat.code) === mat.qty ? '已平衡' : '数量不匹配' }}
            </el-tag>
          </div>
        </el-card>
      </div>
    </div>

    <!-- ============================================================ -->
    <!-- Step 5: 确认提交                                              -->
    <!-- ============================================================ -->
    <div v-if="activeStep === 4" class="step-content">
      <el-card shadow="never">
        <template #header>
          <span>订单确认摘要</span>
        </template>

        <el-table :data="summaryData" border stripe style="width: 100%">
          <el-table-column type="index" label="#" width="50" align="center" />
          <el-table-column prop="materialCode" label="物料编码" width="110" />
          <el-table-column prop="materialName" label="物料名称" min-width="200" show-overflow-tooltip />
          <el-table-column prop="qty" label="数量" width="100" align="right">
            <template #default="{ row }">
              {{ row.qty.toLocaleString() }}
            </template>
          </el-table-column>
          <el-table-column prop="unit" label="单位" width="60" align="center" />
          <el-table-column label="供应商及分配" min-width="240">
            <template #default="{ row }">
              <div v-for="sup in row.suppliers" :key="sup.supplierId" class="summary-supplier-item">
                <span>{{ sup.supplierName }}：</span>
                <span>{{ sup.qty.toLocaleString() }} 个</span>
                <span class="summary-price">@ ¥{{ sup.price.toFixed(2) }}</span>
              </div>
            </template>
          </el-table-column>
          <el-table-column label="金额" width="140" align="right">
            <template #default="{ row }">
              ¥{{ row.totalAmount.toFixed(2) }}
            </template>
          </el-table-column>
        </el-table>

        <!-- Grand Total -->
        <div class="grand-total-row">
          <span class="grand-total-label">合计金额：</span>
          <span class="grand-total-value">¥{{ grandTotal.toFixed(2) }}</span>
        </div>
      </el-card>
    </div>

    <!-- ============================================================ -->
    <!-- Navigation Footer                                             -->
    <!-- ============================================================ -->
    <div class="nav-footer">
      <el-button @click="cancel">
        <el-icon><Close /></el-icon>
        取消
      </el-button>
      <div class="nav-footer-right">
        <el-button v-if="activeStep > 0" @click="prevStep">
          <el-icon><ArrowLeft /></el-icon>
          上一步
        </el-button>
        <el-button
          v-if="activeStep < 4"
          type="primary"
          @click="nextStep"
        >
          下一步
          <el-icon><ArrowRight /></el-icon>
        </el-button>
        <el-button
          v-if="activeStep === 4"
          type="primary"
          :loading="submitting"
          @click="submitOrder"
        >
          <el-icon><Check /></el-icon>
          提交审批
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import {
  ArrowLeft,
  ArrowRight,
  CopyDocument,
  DocumentCopy,
  Check,
  Close,
} from '@element-plus/icons-vue'
import { ElMessage, ElMessageBox } from 'element-plus'

// ── Route ──────────────────────────────────────────────────
const router = useRouter()
const route = useRoute()
const reqId = computed(() => route.params.reqId || '')

// ── Mock Data ──────────────────────────────────────────────

const materials = ref([
  {
    id: 1,
    code: 'CTN-001',
    name: '三层瓦楞纸箱 400x300x250mm',
    spec: '400×300×250mm B楞',
    qty: 30000,
    unit: '个',
    isNew: false,
    oaPrice: null,
  },
  {
    id: 2,
    code: 'CTN-002',
    name: '五层瓦楞纸箱 600x400x350mm',
    spec: '600×400×350mm BC楞',
    qty: 25000,
    unit: '个',
    isNew: false,
    oaPrice: null,
  },
  {
    id: 3,
    code: 'CTN-003',
    name: '重型纸箱 800x600x500mm',
    spec: '800×600×500mm 三A楞',
    qty: 15000,
    unit: '个',
    isNew: true,
    oaPrice: 12.80,
  },
  {
    id: 4,
    code: 'CTN-004',
    name: '食品级纸箱 500x350x300mm',
    spec: '500×350×300mm E楞 食品级',
    qty: 10000,
    unit: '个',
    isNew: false,
    oaPrice: null,
  },
  {
    id: 5,
    code: 'CTN-005',
    name: '防水纸箱 450x300x280mm',
    spec: '450×300×280mm 防水处理',
    qty: 5000,
    unit: '个',
    isNew: true,
    oaPrice: 9.50,
  },
])

const suppliers = ref([
  { id: 'SUP001', name: '浙江优质纸箱', grade: 'A级', pkg: '标包A' },
  { id: 'SUP002', name: '广州精工包装', grade: 'B级', pkg: '标包A' },
  { id: 'SUP003', name: '上海环保纸品', grade: 'A级', pkg: '标包A' },
  { id: 'SUP004', name: '北京新材料包装', grade: 'B级', pkg: '标包B' },
  { id: 'SUP005', name: '深圳精密包装', grade: 'B级', pkg: '标包B' },
])

const quarterlyQuotes = ref([
  { id: 1, materialCode: 'CTN-001', supplierId: 'SUP001', supplierName: '浙江优质纸箱', price: 3.50, pkg: '标包A', grade: 'A级' },
  { id: 2, materialCode: 'CTN-001', supplierId: 'SUP002', supplierName: '广州精工包装', price: 3.20, pkg: '标包A', grade: 'B级' },
  { id: 3, materialCode: 'CTN-001', supplierId: 'SUP003', supplierName: '上海环保纸品', price: 3.80, pkg: '标包A', grade: 'A级' },
  { id: 4, materialCode: 'CTN-002', supplierId: 'SUP001', supplierName: '浙江优质纸箱', price: 5.20, pkg: '标包A', grade: 'A级' },
  { id: 5, materialCode: 'CTN-002', supplierId: 'SUP003', supplierName: '上海环保纸品', price: 5.50, pkg: '标包A', grade: 'A级' },
  { id: 6, materialCode: 'CTN-002', supplierId: 'SUP005', supplierName: '深圳精密包装', price: 5.00, pkg: '标包B', grade: 'B级' },
  { id: 7, materialCode: 'CTN-004', supplierId: 'SUP002', supplierName: '广州精工包装', price: 2.80, pkg: '标包A', grade: 'B级' },
  { id: 8, materialCode: 'CTN-004', supplierId: 'SUP004', supplierName: '北京新材料包装', price: 2.60, pkg: '标包B', grade: 'B级' },
  { id: 9, materialCode: 'CTN-004', supplierId: 'SUP005', supplierName: '深圳精密包装', price: 2.90, pkg: '标包B', grade: 'B级' },
])

const supplierReferenceData = {
  SUP001: { location: '浙江省', isStandardSample: true, hasBulkHistory: true, lastAllocationQty: 20000 },
  SUP002: { location: '广东省', isStandardSample: true, hasBulkHistory: true, lastAllocationQty: 15000 },
  SUP003: { location: '上海市', isStandardSample: false, hasBulkHistory: true, lastAllocationQty: 18000 },
  SUP004: { location: '北京市', isStandardSample: true, hasBulkHistory: false, lastAllocationQty: 0 },
  SUP005: { location: '广东省', isStandardSample: false, hasBulkHistory: false, lastAllocationQty: 0 },
}

// ── Quality Select Options ────────────────────────────────

const testResultOptions = [
  { label: '合格', value: '合格' },
  { label: '不合格', value: '不合格' },
  { label: '未检测', value: '未检测' },
]

const colorDiffOptions = [
  { label: '无色差', value: '无色差' },
  { label: '轻微色差', value: '轻微色差' },
  { label: '一般色差', value: '一般色差' },
  { label: '较重色差', value: '较重色差' },
  { label: '严重色差', value: '严重色差' },
]

const qualityOptions = [
  { label: '完美', value: '完美' },
  { label: '非常好', value: '非常好' },
  { label: '良好', value: '良好' },
  { label: '一般', value: '一般' },
  { label: '糟糕', value: '糟糕' },
]

// ── Wizard State ──────────────────────────────────────────

const activeStep = ref(0)
const submitting = ref(false)

// ── Step 1 State: Pricing & Quotes ────────────────────────

const pricingStrategy = reactive({})
const selectedQuoteId = reactive({})

function getMaterialQuotes(matCode) {
  return quarterlyQuotes.value.filter((q) => q.materialCode === matCode)
}

function onStrategyChange(matCode, strategy) {
  if (strategy === '最低价') {
    autoSelectLowest(matCode)
  }
}

function autoSelectLowest(matCode) {
  const quotes = getMaterialQuotes(matCode)
  if (quotes.length > 0) {
    const lowest = quotes.reduce((a, b) => (a.price < b.price ? a : b))
    selectedQuoteId[matCode] = lowest.id
  }
}

function selectQuote(matCode, quote) {
  if (pricingStrategy[matCode] === '最低价') {
    ElMessage.info('当前为最低价策略，请切换至"自定义"以手动选择')
    return
  }
  if (selectedQuoteId[matCode] === quote.id) {
    selectedQuoteId[matCode] = null
  } else {
    selectedQuoteId[matCode] = quote.id
  }
}

function getPriceForMaterial(matCode) {
  const strategy = pricingStrategy[matCode]
  const mat = materials.value.find((m) => m.code === matCode)
  if (strategy === '自定义') {
    const qId = selectedQuoteId[matCode]
    if (qId) {
      const quote = quarterlyQuotes.value.find((q) => q.id === qId)
      return quote?.price ?? 0
    }
    return 0
  }
  // 最低价 — find the minimum price
  const quotes = getMaterialQuotes(matCode)
  if (quotes.length > 0) {
    return Math.min(...quotes.map((q) => q.price))
  }
  if (mat?.isNew && mat?.oaPrice) {
    return mat.oaPrice
  }
  return 0
}

// ── Step 2 State: Supplier Selection & Quality ────────────

const supplierSelections = reactive({})

function onSupplierSelectChange(matCode, supId) {
  // Triggered when checkbox changes — all logic is in v-model binding
}

// ── Step 3 State: Card Paper Config ───────────────────────

const cardPaperConfig = reactive({})
const clipboardCardPaper = ref('')

function copyCardPaper(code) {
  clipboardCardPaper.value = cardPaperConfig[code] || ''
  ElMessage.success(`已复制 ${code} 的卡纸配置`)
}

function pasteCardPaper(code) {
  if (clipboardCardPaper.value) {
    cardPaperConfig[code] = clipboardCardPaper.value
    ElMessage.success(`已粘贴到 ${code}`)
  }
}

function copyAllCardPaper() {
  const allConfigs = materials.value
    .map((m) => `${m.code}:${cardPaperConfig[m.code] || ''}`)
    .join('||')
  clipboardCardPaper.value = allConfigs
  ElMessage.success('已复制全部卡纸配置')
}

function pasteAllCardPaper() {
  if (!clipboardCardPaper.value) return
  const sep = clipboardCardPaper.value.includes('||') ? '||' : ''
  if (sep) {
    const parts = clipboardCardPaper.value.split('||')
    for (const part of parts) {
      const [code, ...rest] = part.split(':')
      const val = rest.join(':')
      if (code && materials.value.some((m) => m.code === code)) {
        cardPaperConfig[code] = val
      }
    }
  } else {
    for (const mat of materials.value) {
      cardPaperConfig[mat.code] = clipboardCardPaper.value
    }
  }
  ElMessage.success('已粘贴到全部物料')
}

function clearAllCardPaper() {
  for (const mat of materials.value) {
    cardPaperConfig[mat.code] = ''
  }
  ElMessage.success('已清空全部卡纸配置')
}

// ── Step 4 State: Quantity Allocation ─────────────────────

const allocationData = reactive({})

function getAllocationRows(matCode) {
  const rows = []
  const selections = supplierSelections[matCode] || {}
  for (const [supId, sel] of Object.entries(selections)) {
    if (!sel.selected) continue
    if (!allocationData[matCode]) {
      allocationData[matCode] = {}
    }
    if (!allocationData[matCode][supId]) {
      allocationData[matCode][supId] = { qty: 0, remark: '' }
    }
    const sup = suppliers.value.find((s) => s.id === supId)
    const refData = supplierReferenceData[supId] || {}
    rows.push({
      materialCode: matCode,
      supplierId: supId,
      supplierName: sup?.name || supId,
      pkg: sup?.pkg || '',
      location: refData.location || '',
      isStandardSample: refData.isStandardSample ?? false,
      hasBulkHistory: refData.hasBulkHistory ?? false,
      lastAllocationQty: refData.lastAllocationQty ?? 0,
    })
  }
  return rows
}

function getAllocationTotal(matCode) {
  const data = allocationData[matCode] || {}
  return Object.values(data).reduce((sum, item) => sum + (Number(item.qty) || 0), 0)
}

function getRequiredQty(matCode) {
  const mat = materials.value.find((m) => m.code === matCode)
  return mat?.qty || 0
}

// ── Step 5 State: Summary Computation ─────────────────────

const summaryData = computed(() => {
  return materials.value.map((mat) => {
    const price = getPriceForMaterial(mat.code)
    const rows = getAllocationRows(mat.code)
    const suppliersSummary = rows.map((r) => {
      const alloc = allocationData[r.materialCode]?.[r.supplierId]
      const allocQty = alloc?.qty || 0
      return {
        supplierId: r.supplierId,
        supplierName: r.supplierName,
        qty: allocQty,
        price: price,
        amount: allocQty * price,
      }
    })
    const totalAmount = suppliersSummary.reduce((s, d) => s + d.amount, 0)
    return {
      materialCode: mat.code,
      materialName: mat.name,
      spec: mat.spec,
      qty: mat.qty,
      unit: mat.unit,
      suppliers: suppliersSummary,
      totalAmount: totalAmount,
    }
  })
})

const grandTotal = computed(() => {
  return summaryData.value.reduce((sum, item) => sum + item.totalAmount, 0)
})

// ── Step Validation ───────────────────────────────────────

function validateStep() {
  switch (activeStep.value) {
    case 0: {
      for (const mat of materials.value) {
        if (!pricingStrategy[mat.code]) {
          ElMessage.warning(`请为 ${mat.code} 选择定价策略`)
          return false
        }
        const hasQuotes = quarterlyQuotes.value.some(
          (q) => q.materialCode === mat.code
        )
        if (hasQuotes && !selectedQuoteId[mat.code]) {
          ElMessage.warning(`请为 ${mat.code} 选择供应商报价`)
          return false
        }
      }
      return true
    }
    case 1: {
      for (const mat of materials.value) {
        const selections = supplierSelections[mat.code] || {}
        const hasSelected = Object.values(selections).some((s) => s.selected)
        if (!hasSelected) {
          ElMessage.warning(`请为 ${mat.code} 选择至少一个供应商`)
          return false
        }
      }
      return true
    }
    case 2: {
      // Card paper is optional — no validation needed
      return true
    }
    case 3: {
      for (const mat of materials.value) {
        const total = getAllocationTotal(mat.code)
        if (total !== mat.qty) {
          ElMessage.warning(
            `${mat.code} 分配数量 (${total.toLocaleString()}) 与需求数量 (${mat.qty.toLocaleString()}) 不匹配`
          )
          return false
        }
      }
      return true
    }
    default:
      return true
  }
}

// ── Navigation ────────────────────────────────────────────

function nextStep() {
  if (!validateStep()) return
  if (activeStep.value < 4) {
    activeStep.value++
  }
}

function prevStep() {
  if (activeStep.value > 0) {
    activeStep.value--
  }
}

function cancel() {
  ElMessageBox.confirm('确定取消创建采购订单吗？已填写的数据将丢失。', '提示', {
    confirmButtonText: '确定',
    cancelButtonText: '继续填写',
    type: 'warning',
  })
    .then(() => {
      router.push('/procurement/carton')
    })
    .catch(() => {})
}

// ── Submit ────────────────────────────────────────────────

function submitOrder() {
  ElMessageBox.confirm(
    '确认提交该采购订单进行审批？',
    '提交审批',
    {
      confirmButtonText: '确认提交',
      cancelButtonText: '取消',
      type: 'warning',
    }
  )
    .then(() => {
      submitting.value = true
      // Mock API call
      setTimeout(() => {
        submitting.value = false
        ElMessage.success('采购订单已成功提交审批')
        router.push('/procurement/carton/order-list')
      }, 1500)
    })
    .catch(() => {})
}

// ── Initialization ────────────────────────────────────────

onMounted(() => {
  // Initialize supplier selections with defaults
  for (const mat of materials.value) {
    if (!supplierSelections[mat.code]) {
      supplierSelections[mat.code] = {}
    }
    for (const sup of suppliers.value) {
      if (!supplierSelections[mat.code][sup.id]) {
        supplierSelections[mat.code][sup.id] = {
          selected: false,
          testResult: '未检测',
          colorDiff: '无色差',
          quality: '良好',
          score: 3,
        }
      }
    }

    // Initialize pricing strategy and auto-select lowest quote
    pricingStrategy[mat.code] = '最低价'
    autoSelectLowest(mat.code)

    // Initialize card paper config
    cardPaperConfig[mat.code] = ''
  }
})
</script>

<style scoped>
.procurement-container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 20px;
}

/* ── Page Header ──────────────────────────────────────── */

.page-header {
  margin-bottom: 24px;
}

.page-title {
  font-size: 24px;
  font-weight: 600;
  color: #303133;
  margin: 0 0 4px 0;
}

.page-subtitle {
  font-size: 14px;
  color: #909399;
  margin: 0;
}

/* ── Steps ────────────────────────────────────────────── */

.steps-card {
  margin-bottom: 24px;
  border-radius: 8px;
}

/* ── Step Content ─────────────────────────────────────── */

.step-content {
  min-height: 400px;
}

.material-section {
  margin-bottom: 20px;
}

.material-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.material-title-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.material-name {
  color: #606266;
  font-size: 13px;
}

.qty-badge {
  font-size: 12px;
  color: #909399;
  margin-left: 8px;
}

/* ── Step 1: Strategy & Quotes ──────────────────────── */

.strategy-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin: 16px 0;
  padding: 12px 16px;
  background-color: #fafafa;
  border-radius: 6px;
}

.strategy-label {
  font-weight: 500;
  color: #303133;
  white-space: nowrap;
}

.strategy-hint {
  font-size: 12px;
  color: #909399;
}

.quotes-section {
  margin-top: 8px;
}

.quotes-title {
  font-size: 13px;
  font-weight: 500;
  color: #606266;
  margin: 0 0 8px 0;
}

.price-value {
  font-family: 'Courier New', Courier, monospace;
  font-weight: 600;
  color: #f56c6c;
}

.oa-row {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 12px;
  padding: 12px 16px;
  background-color: #fdf6ec;
  border-radius: 6px;
}

.oa-price {
  font-weight: 600;
  color: #e6a23c;
  font-size: 15px;
}

.oa-hint {
  font-size: 12px;
  color: #909399;
}

/* ── Step 2: Supplier Selection ──────────────────────── */

.supplier-card {
  border-radius: 8px;
  transition: box-shadow 0.25s, border-color 0.25s;
  border: 1px solid #ebeef5;
}

.supplier-card.is-selected {
  border-color: var(--el-color-primary);
  box-shadow: 0 0 0 1px var(--el-color-primary);
}

.supplier-card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.supplier-name {
  font-size: 14px;
}

.quality-fields {
  margin-top: 12px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.quality-field {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.field-label {
  font-size: 12px;
  color: #909399;
  font-weight: 500;
}

.score-value {
  font-size: 12px;
  color: #606266;
  margin-left: 4px;
}

/* ── Step 3: Card Paper Config ───────────────────────── */

.card-paper-row {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 16px;
  padding: 8px 0;
}

.card-paper-label {
  min-width: 200px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.card-paper-input {
  flex: 1;
}

.card-paper-actions {
  display: flex;
  gap: 8px;
  white-space: nowrap;
}

.quick-copy-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.quick-copy-label {
  font-size: 13px;
  color: #606266;
  white-space: nowrap;
}

/* ── Step 4: Allocation ────────────────────────────────── */

.allocation-summary {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-top: 12px;
  padding: 10px 16px;
  background-color: #fafafa;
  border-radius: 6px;
  font-size: 14px;
}

.summary-label {
  font-weight: 500;
  color: #303133;
}

/* ── Step 5: Summary ───────────────────────────────────── */

.summary-supplier-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 2px 0;
  font-size: 13px;
}

.summary-price {
  color: #909399;
  font-size: 12px;
}

.grand-total-row {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 20px;
  padding: 16px 24px;
  background-color: #fafafa;
  border-radius: 8px;
}

.grand-total-label {
  font-size: 16px;
  font-weight: 500;
  color: #303133;
}

.grand-total-value {
  font-size: 24px;
  font-weight: 700;
  color: #f56c6c;
  font-family: 'Courier New', Courier, monospace;
}

/* ── Navigation Footer ─────────────────────────────────── */

.nav-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: 24px;
  padding: 16px 0;
  border-top: 1px solid #e4e7ed;
}

.nav-footer-right {
  display: flex;
  gap: 12px;
}
</style>
