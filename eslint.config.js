import { defineConfig } from '@moeru/eslint-config'

export default defineConfig({
  react: true,
}).append({
  rules: {
    'toml/padding-line-between-pairs': 'off',
  },
})
