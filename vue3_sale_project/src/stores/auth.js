import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useAuthStore = defineStore('auth', () => {
  const user = ref(JSON.parse(localStorage.getItem('sale_user') || 'null'))
  const token = ref(localStorage.getItem('sale_token') || '')

  const isLoggedIn = computed(() => !!token.value)

  async function login(username, password) {
    const res = await fetch('/api/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    })
    const json = await res.json()
    if (json.code !== 200) {
      throw new Error(json.message || '登录失败')
    }
    const { user: userInfo, token: newToken } = json.data
    user.value = userInfo
    token.value = newToken
    localStorage.setItem('sale_user', JSON.stringify(userInfo))
    localStorage.setItem('sale_token', newToken)
    return userInfo
  }

  function logout() {
    user.value = null
    token.value = ''
    localStorage.removeItem('sale_user')
    localStorage.removeItem('sale_token')
  }

  return { user, token, isLoggedIn, login, logout }
})
