import { defineConfig, presetIcons, presetUno } from 'unocss'

export default defineConfig({
  presets:[presetUno({
    dark: 'media'
  }), presetIcons()],
  shortcuts:[
    ['l-btn', 'rounded-full p-2 capitalize'],
    ['l-btn-primary', 'bg-amber text-white'],
    ['l-btn-secondary', ' text-amber border border-amber'],
  ],
})
