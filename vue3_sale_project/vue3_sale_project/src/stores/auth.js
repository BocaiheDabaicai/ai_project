import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useAuthStore = defineStore('auth', () => {
  const user = ref(JSON.parse(localStorage.getItem('sale_user') || 'null'))
  const token = ref(localStorage.getItem('sale_token') || '')

  const isLoggedIn = computed(() => !!token.value)

  function login(username, password) {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        if (username === 'admin' && password === '123456') {
          const userInfo = { username, role: '管理员', avatar: '' }
          user.value = userInfo
          token.value = 'mock_token_' + Date.now()
          localStorage.setItem('sale_user', JSON.stringify(userInfo))
          localStorage.setItem('sale_token', token.value)
          resolve(userInfo)
        } else {
          reject(new Error('用户名或密码错误'))
        }
      }, 500)
    })
  }

  function logout() {
    user.value = null
    token.value = ''
    localStorage.removeItem('sale_user')
    localStorage.removeItem('sale_token')
  }

  return { user, token, isLoggedIn, login, logout }
})
