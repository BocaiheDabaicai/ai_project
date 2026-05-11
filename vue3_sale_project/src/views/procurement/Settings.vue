<script setup>
import { ref } from 'vue'
import { ElMessage } from 'element-plus'

// ===== 负责采购的物料类型 =====
const materialTypes = ref([
  { id: 1, name: '金属材料', category: '原材料', status: true, remark: '板材、线材、型材等' },
  { id: 2, name: '电子元器件', category: '零部件', status: true, remark: '芯片、电阻、电容等' },
  { id: 3, name: '紧固件', category: '标准件', status: true, remark: '螺栓、螺母、垫圈等' },
  { id: 4, name: '电气材料', category: '原材料', status: true, remark: '电缆、开关、配电等' },
  { id: 5, name: '化工辅料', category: '辅料', status: false, remark: '润滑油、清洗剂等' },
  { id: 6, name: '包装材料', category: '辅料', status: false, remark: '纸箱、泡沫、胶带等' },
])

function toggleMaterialStatus(item) {
  item.status = !item.status
  ElMessage.success(`已${item.status ? '启用' : '停用'}物料类型：${item.name}`)
}

// ===== 标包配置 =====
const packages = ref([
  { id: 1, code: 'PKG-A', name: '标包A', description: '金属材料类采购标包（板材、型材）', suppliers: ['浙江优质供应商有限公司', '广州精工制造有限公司'], materialTypes: ['金属材料'], status: '启用' },
  { id: 2, code: 'PKG-B', name: '标包B', description: '紧固件类采购标包（螺栓、螺母）', suppliers: ['浙江优质供应商有限公司', '北京新材料科技有限公司'], materialTypes: ['紧固件'], status: '启用' },
  { id: 3, code: 'PKG-C', name: '标包C', description: '电气材料类采购标包（电缆、配电）', suppliers: ['广州精工制造有限公司', '上海电子配件有限公司'], materialTypes: ['电气材料'], status: '启用' },
])

const pkgDialog = ref(false)
const editPkg = ref(null)
const isNewPkg = ref(false)

function openAddPkg() {
  isNewPkg.value = true
  editPkg.value = { code: '', name: '', description: '', suppliers: [], materialTypes: [], status: '启用' }
  pkgDialog.value = true
}

function openEditPkg(pkg) {
  isNewPkg.value = false
  editPkg.value = { ...pkg }
  pkgDialog.value = true
}

function savePkg() {
  if (isNewPkg.value) {
    editPkg.value.id = packages.value.length + 1
    packages.value.push({ ...editPkg.value })
    ElMessage.success('标包已创建')
  } else {
    const idx = packages.value.findIndex(p => p.id === editPkg.value.id)
    if (idx !== -1) {
      packages.value[idx] = { ...editPkg.value }
      ElMessage.success('标包已更新')
    }
  }
  pkgDialog.value = false
}

// ===== 供应商管理（简化） =====
const supplierList = ref([
  { id: 1, code: 'SUP001', name: '浙江优质供应商有限公司', level: 'A级', packages: ['标包A', '标包B'], status: '合作中' },
  { id: 2, code: 'SUP002', name: '广州精工制造有限公司', level: 'B级', packages: ['标包A', '标包C'], status: '合作中' },
  { id: 3, code: 'SUP003', name: '上海电子配件有限公司', level: 'A级', packages: ['标包C'], status: '合作中' },
  { id: 4, code: 'SUP004', name: '北京新材料科技有限公司', level: 'B级', packages: ['标包B'], status: '评估中' },
])
</script>

<template>
  <div class="page-container">
    <h2 class="page-title">
      <el-icon :size="20" color="var(--color-accent)"><Setting /></el-icon>
      采购配置
    </h2>

    <!-- ===== 负责采购的物料类型 ===== -->
    <el-card shadow="never" style="margin-bottom:20px">
      <template #header>
        <div style="display:flex;justify-content:space-between;align-items:center">
          <span><strong>负责采购的物料类型</strong></span>
        </div>
      </template>
      <p style="font-size:13px;color:var(--text-secondary);margin-bottom:12px">配置采购人员负责的物料类型，未启用的类型不会出现在请购单的自动勾选列表中。</p>
      <div class="table-wrapper">
        <table class="data-table">
          <thead>
            <tr><th>物料类型</th><th>所属大类</th><th>备注</th><th>状态</th><th>操作</th></tr>
          </thead>
          <tbody>
            <tr v-for="mt in materialTypes" :key="mt.id">
              <td><strong>{{ mt.name }}</strong></td>
              <td>{{ mt.category }}</td>
              <td>{{ mt.remark }}</td>
              <td>
                <el-tag :type="mt.status ? 'success' : 'info'" effect="light" size="small" round>{{ mt.status ? '启用' : '停用' }}</el-tag>
              </td>
              <td>
                <el-button :type="mt.status ? 'warning' : 'success'" size="small" @click="toggleMaterialStatus(mt)">
                  {{ mt.status ? '停用' : '启用' }}
                </el-button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </el-card>

    <!-- ===== 标包管理 ===== -->
    <el-card shadow="never" style="margin-bottom:20px">
      <template #header>
        <div style="display:flex;justify-content:space-between;align-items:center">
          <span><strong>标包配置</strong></span>
          <el-button type="primary" size="small" :icon="Plus" @click="openAddPkg">新增标包</el-button>
        </div>
      </template>
      <p style="font-size:13px;color:var(--text-secondary);margin-bottom:12px">标包用于在比价和分配数量阶段确定供应商范围，同一标包内的供应商可以对相应物料进行竞价和供货。</p>
      <div class="table-wrapper">
        <table class="data-table">
          <thead>
            <tr><th>标包编码</th><th>标包名称</th><th>描述</th><th>覆盖物料</th><th>关联供应商</th><th>状态</th><th>操作</th></tr>
          </thead>
          <tbody>
            <tr v-for="pkg in packages" :key="pkg.id">
              <td><span class="code-text">{{ pkg.code }}</span></td>
              <td>{{ pkg.name }}</td>
              <td>{{ pkg.description }}</td>
              <td>
                <el-tag v-for="mt in pkg.materialTypes" :key="mt" size="small" style="margin:1px">{{ mt }}</el-tag>
              </td>
              <td>
                <el-tag v-for="s in pkg.suppliers" :key="s" size="small" effect="plain" style="margin:1px">{{ s }}</el-tag>
              </td>
              <td>
                <el-tag :type="pkg.status === '启用' ? 'success' : 'info'" effect="light" size="small" round>{{ pkg.status }}</el-tag>
              </td>
              <td>
                <el-button type="primary" link size="small" @click="openEditPkg(pkg)">编辑</el-button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </el-card>

    <!-- ===== 供应商概览 ===== -->
    <el-card shadow="never">
      <template #header>
        <span><strong>供应商概览</strong></span>
      </template>
      <div class="table-wrapper">
        <table class="data-table">
          <thead>
            <tr><th>编号</th><th>名称</th><th>等级</th><th>关联标包</th><th>状态</th></tr>
          </thead>
          <tbody>
            <tr v-for="s in supplierList" :key="s.id">
              <td><span class="code-text">{{ s.code }}</span></td>
              <td>{{ s.name }}</td>
              <td><el-tag :type="s.level === 'A级' ? 'success' : ''" effect="light" size="small" round>{{ s.level }}</el-tag></td>
              <td>
                <el-tag v-for="p in s.packages" :key="p" size="small" type="warning" style="margin:1px">{{ p }}</el-tag>
              </td>
              <td><el-tag :type="s.status === '合作中' ? 'success' : 'warning'" effect="light" size="small" round>{{ s.status }}</el-tag></td>
            </tr>
          </tbody>
        </table>
      </div>
    </el-card>

    <!-- ===== 标包编辑弹窗 ===== -->
    <el-dialog v-model="pkgDialog" :title="isNewPkg ? '新增标包' : '编辑标包'" width="500px">
      <el-form :model="editPkg" label-width="80px">
        <el-form-item label="标包编码">
          <el-input v-model="editPkg.code" placeholder="如 PKG-A" />
        </el-form-item>
        <el-form-item label="标包名称">
          <el-input v-model="editPkg.name" placeholder="如 标包A" />
        </el-form-item>
        <el-form-item label="描述">
          <el-input v-model="editPkg.description" type="textarea" :rows="2" />
        </el-form-item>
        <el-form-item label="状态">
          <el-radio-group v-model="editPkg.status">
            <el-radio label="启用">启用</el-radio>
            <el-radio label="停用">停用</el-radio>
          </el-radio-group>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="pkgDialog = false">取消</el-button>
        <el-button type="primary" @click="savePkg">保存</el-button>
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
