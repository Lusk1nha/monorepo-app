import { defineConfig } from 'tsup'

export default defineConfig({
  entry: ['src/index.ts'],
  dts: true,
  format: ['esm', 'cjs'],
  external: ['react'],
  onSuccess: 'postcss src/styles.css -o dist/styles.css',
})
