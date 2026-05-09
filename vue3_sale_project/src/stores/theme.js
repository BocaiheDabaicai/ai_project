import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

const THEME_KEY = 'sale_theme'
const THEMES = [
  { key: 'sky-blue', label: '天空蓝', color: '#3498db' },
  { key: 'mint-green', label: '薄荷绿', color: '#1abc9c' },
  { key: 'warm-orange', label: '暖阳橙', color: '#f39c12' },
  { key: 'violet', label: '紫罗兰', color: '#8e44ad' },
  { key: 'minimal-white', label: '极简白', color: '#636e72' }
]

export const useThemeStore = defineStore('theme', () => {
  const current = ref(localStorage.getItem(THEME_KEY) || 'sky-blue')

  function applyTheme(key) {
    current.value = key
    document.documentElement.setAttribute('data-theme', key)
    localStorage.setItem(THEME_KEY, key)
  }

  function initTheme() {
    applyTheme(current.value)
  }

  return { current, themes: THEMES, applyTheme, initTheme }
})
