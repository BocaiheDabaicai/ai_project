<script setup>
import { ref, reactive } from 'vue'

const filters = reactive({ orderNo: '', supplier: '', status: '' })

const orders = ref([
  { id: 1, orderNo: 'PO20260501001', supplier: '浙江优质供应商有限公司', amount: 156800.00, quantity: 12, status: '已完成', createTime: '2026-05-01 09:30' },
  { id: 2, orderNo: 'PO20260503002', supplier: '广州精工制造有限公司', amount: 89300.50, quantity: 8, status: '审批中', createTime: '2026-05-03 14:20' },
  { id: 3, orderNo: 'PO20260504003', supplier: '上海电子配件有限公司', amount: 24500.00, quantity: 5, status: '待发货', createTime: '2026-05-04 10:00' },
  { id: 4, orderNo: 'PO20260505004', supplier: '北京新材料科技有限公司', amount: 432200.00, quantity: 20, status: '审批中', createTime: '2026-05-05 16:45' },
  { id: 5, orderNo: 'PO20260506005', supplier: '浙江优质供应商有限公司', amount: 78000.00, quantity: 15, status: '已完成', createTime: '2026-05-06 08:30' }
])

function statusType(s) {
  return { '已完成': 'success', '审批中': 'warning', '待发货': '' }[s] || 'info'
}

function handleSearch() {}
function handleReset() {
  filters.orderNo = ''
  filters.supplier = ''
  filters.status = ''
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
      <el-input v-model="filters.orderNo" placeholder="订单编号" clearable style="width:180px" />
      <el-input v-model="filters.supplier" placeholder="供应商名称" clearable style="width:200px" />
      <el-select v-model="filters.status" placeholder="全部状态" clearable style="width:130px">
        <el-option label="已完成" value="已完成" />
        <el-option label="审批中" value="审批中" />
        <el-option label="待发货" value="待发货" />
      </el-select>
      <el-button type="primary" :icon="Search" @click="handleSearch">查询</el-button>
      <el-button :icon="RefreshRight" @click="handleReset">重置</el-button>
      <el-button type="success" :icon="Plus" style="margin-left:auto">新建采购订单</el-button>
    </div>

    <!-- 数据表格 -->
    <div class="table-wrapper">
      <table class="data-table">
        <thead>
          <tr>
            <th>#</th>
            <th>订单编号</th>
            <th>供应商名称</th>
            <th>采购金额(元)</th>
            <th>采购数量</th>
            <th>状态</th>
            <th>创建时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(item, idx) in orders" :key="item.id">
            <td>{{ idx + 1 }}</td>
            <td><span class="order-no">{{ item.orderNo }}</span></td>
            <td>{{ item.supplier }}</td>
            <td>¥{{ item.amount.toLocaleString() }}</td>
            <td>{{ item.quantity }}</td>
            <td>
              <el-tag :type="statusType(item.status)" effect="light" size="small" round>
                {{ item.status }}
              </el-tag>
            </td>
            <td>{{ item.createTime }}</td>
            <td>
              <el-button type="primary" link size="small">查看</el-button>
              <el-button type="primary" link size="small">编辑</el-button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- 分页 -->
    <div class="pagination">
      <span>共 {{ orders.length }} 条记录</span>
      <div class="page-actions">
        <button disabled>&lt;</button>
        <button class="active">1</button>
        <button>&gt;</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.order-no {
  font-family: 'SF Mono', 'Consolas', 'Monaco', monospace;
  font-size: 12px;
  color: var(--color-accent);
}
</style>
