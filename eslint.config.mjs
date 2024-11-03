import config from '@antfu/eslint-config'

export default config({
  rules: {
    'vue/custom-event-casing': ['warn', 'kebab-case']
  }
})
