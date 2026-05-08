<script setup>
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()

const formRef = ref(null)
const form = ref({ username: 'admin', password: '123456' })
const loading = ref(false)

const rules = {
  username: [{ required: true, message: '请输入用户名', trigger: 'blur' }],
  password: [{ required: true, message: '请输入密码', trigger: 'blur' }]
}

async function handleLogin() {
  const valid = await formRef.value?.validate().catch(() => false)
  if (!valid) return
  loading.value = true
  try {
    await authStore.login(form.value.username, form.value.password)
    const redirect = route.query.redirect || '/'
    router.push(redirect)
  } catch {
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login-container">
    <div class="login-bg-decoration">
      <div class="circle c1"></div>
      <div class="circle c2"></div>
      <div class="circle c3"></div>
    </div>

    <div class="login-card">
      <div class="login-card-header">
        <div class="login-icon">
          <el-icon :size="32" color="#fff"><ShoppingCartFull /></el-icon>
        </div>
        <h1 class="login-title">采购运营平台</h1>
        <p class="login-subtitle">Procurement Operations Platform</p>
      </div>

      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        class="login-form"
        @keyup.enter="handleLogin"
      >
        <el-form-item prop="username">
          <el-input
            v-model="form.username"
            placeholder="请输入用户名"
            :prefix-icon="User"
            size="large"
            clearable
          />
        </el-form-item>
        <el-form-item prop="password">
          <el-input
            v-model="form.password"
            type="password"
            placeholder="请输入密码"
            :prefix-icon="Lock"
            size="large"
            show-password
          />
        </el-form-item>

        <el-button
          type="primary"
          size="large"
          class="login-btn"
          :loading="loading"
          @click="handleLogin"
        >
          {{ loading ? '登录中...' : '登 录' }}
        </el-button>

        <p class="login-hint">默认账号: admin &nbsp;|&nbsp; 密码: 123456</p>
      </el-form>
    </div>
  </div>
</template>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--gradient-login);
  position: relative;
  overflow: hidden;
}

.login-bg-decoration .circle {
  position: absolute;
  border-radius: 50%;
  opacity: .15;
  background: #fff;
  animation: float 6s ease-in-out infinite;
}
.login-bg-decoration .c1 {
  width: 300px; height: 300px;
  top: -80px; left: -60px;
  animation-delay: 0s;
}
.login-bg-decoration .c2 {
  width: 200px; height: 200px;
  bottom: -40px; right: 10%;
  animation-delay: 2s;
}
.login-bg-decoration .c3 {
  width: 150px; height: 150px;
  top: 40%; right: -40px;
  animation-delay: 4s;
}

@keyframes float {
  0%, 100% { transform: translateY(0) scale(1); }
  50% { transform: translateY(-20px) scale(1.05); }
}

.login-card {
  width: 420px;
  padding: 44px 40px 36px;
  background: var(--bg-card);
  border-radius: 16px;
  box-shadow: var(--shadow-lg);
  position: relative;
  z-index: 1;
}

.login-card-header {
  text-align: center;
  margin-bottom: 32px;
}
.login-icon {
  width: 60px;
  height: 60px;
  margin: 0 auto 16px;
  background: var(--gradient-btn);
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 8px 20px rgba(0,0,0,.12);
}

.login-title {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
  margin-bottom: 4px;
  letter-spacing: 1px;
}
.login-subtitle {
  font-size: 13px;
  color: var(--text-muted);
  letter-spacing: 2px;
  text-transform: uppercase;
}

.login-form :deep(.el-input__wrapper) {
  box-shadow: 0 0 0 1px var(--border-color) inset;
  border-radius: var(--radius-sm);
}
.login-form :deep(.el-input__wrapper:hover) {
  box-shadow: 0 0 0 1px var(--color-accent) inset;
}

.login-btn {
  width: 100%;
  margin-top: 4px;
  height: 44px;
  font-size: 16px;
  letter-spacing: 4px;
  border-radius: var(--radius-sm);
  border: none;
  background: var(--gradient-btn) !important;
  transition: opacity .3s;
}
.login-btn:hover {
  opacity: .88;
}
.login-btn:active {
  opacity: .75;
}

.login-hint {
  text-align: center;
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 20px;
}
</style>
