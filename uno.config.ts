import { defineConfig, presetIcons, presetUno } from 'unocss'

export default defineConfig({
  presets:[presetUno({
    dark: 'media'
  }), presetIcons()],
  shortcuts:[
    ['l-btn', 'rounded-full p-2 capitalize'],
    ['l-btn-primary', 'bg-amber text-white'],
    ['l-btn-secondary', ' text-amber border border-amber'],
    ['success-prose', 'text-green-4 @dark:text-green-4'],
    ['success-bg', 'bg-green-50 text-green-5 @dark:bg-green-5 @dark:text-green-50 hover:bg-green-6 hover:text-white'],
    ['warning-prose', 'text-yellow-5 @dark:text-yellow-3'],
    ['warning-bg', 'bg-yellow-50 text-yellow-5 @dark:bg-yellow-5 @dark:text-yellow-50 hover:bg-yellow-6 hover:text-white'],
    ['info-prose', 'text-blue-5 @dark:text-blue-3'],
    ['info-bg', 'bg-blue-50 text-blue-5 @dark:bg-blue-5 @dark:text-blue-50 hover:bg-blue-6 hover:text-white'],
    ['danger-prose', 'text-red-5 @dark:text-red-3'],
    ['danger-bg', 'bg-red-50 text-red-5 @dark:bg-red-5 @dark:text-red-50 hover:bg-red-6 hover:text-white'],
  ],
})
