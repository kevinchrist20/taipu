import config from '@antfu/eslint-config'

export default config({
  rules: {
    'vue/custom-event-name-casing': ['warn', 'kebab-case'],
  },
})
