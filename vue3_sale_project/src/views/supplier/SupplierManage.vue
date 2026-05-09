<script setup>
import { ref } from 'vue'

const suppliers = ref([
  { id: 1, code: 'SUP001', name: '浙江优质供应商有限公司', contact: '张经理', phone: '0571-8888xxxx', level: 'A级', cooperation: 5, status: '合作中' },
  { id: 2, code: 'SUP002', name: '广州精工制造有限公司', contact: '李主管', phone: '020-6666xxxx', level: 'B级', cooperation: 3, status: '合作中' },
  { id: 3, code: 'SUP003', name: '上海电子配件有限公司', contact: '王总', phone: '021-5555xxxx', level: 'A级', cooperation: 8, status: '合作中' },
  { id: 4, code: 'SUP004', name: '北京新材料科技有限公司', contact: '赵经理', phone: '010-7777xxxx', level: 'B级', cooperation: 2, status: '评估中' },
  { id: 5, code: 'SUP005', name: '深圳精密机械有限公司', contact: '陈主管', phone: '0755-9999xxxx', level: 'A级', cooperation: 6, status: '合作中' }
])

function levelType(l) { return l === 'A级' ? 'success' : '' }
function statusType(s) { return s === '合作中' ? 'success' : s === '评估中' ? 'warning' : 'danger' }
</script>

<template>
  <div class="page-container">
    <h2 class="page-title">
      <el-icon :size="20" color="var(--color-accent)"><OfficeBuilding /></el-icon>
      供应商管理
    </h2>

    <!-- 统计卡片 -->
    <div class="stats-row">
      <div class="stat-card">
        <div class="stat-value">{{ suppliers.length }}</div>
        <div class="stat-label">供应商总数</div>
      </div>
      <div class="stat-card">
        <div class="stat-value" style="color:#67c23a">{{ suppliers.filter(s => s.status === '合作中').length }}</div>
        <div class="stat-label">合作中</div>
      </div>
      <div class="stat-card">
        <div class="stat-value" style="color:var(--color-accent)">{{ suppliers.filter(s => s.level === 'A级').length }}</div>
        <div class="stat-label">A级供应商</div>
      </div>
      <div class="stat-card">
        <div class="stat-value" style="color:#e6a23c">{{ suppliers.filter(s => s.status === '评估中').length }}</div>
        <div class="stat-label">评估中</div>
      </div>
      <el-button type="success" :icon="Plus" style="align-self:center;height:40px">新增供应商</el-button>
    </div>

    <!-- 数据表格 -->
    <div class="table-wrapper">
      <table class="data-table">
        <thead>
          <tr>
            <th>#</th>
            <th>供应商编号</th>
            <th>供应商名称</th>
            <th>联系人</th>
            <th>联系电话</th>
            <th>等级</th>
            <th>合作年限(年)</th>
            <th>合作状态</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(item, idx) in suppliers" :key="item.id">
            <td>{{ idx + 1 }}</td>
            <td><span class="code-text">{{ item.code }}</span></td>
            <td>{{ item.name }}</td>
            <td>{{ item.contact }}</td>
            <td>{{ item.phone }}</td>
            <td>
              <el-tag :type="levelType(item.level)" effect="light" size="small" round>
                {{ item.level }}
              </el-tag>
            </td>
            <td>{{ item.cooperation }}</td>
            <td>
              <el-tag :type="statusType(item.status)" effect="light" size="small" round>
                {{ item.status }}
              </el-tag>
            </td>
            <td>
              <el-button type="primary" link size="small">详情</el-button>
              <el-button type="primary" link size="small">编辑</el-button>
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
