import { defineConfig } from 'vite'

export default defineConfig({
  base: '',
  define: {
    global: 'globalThis',
  },
  worker: {
    format: 'es'
  },
  optimizeDeps: {
    include: [
      'monaco-editor/esm/vs/language/json/json.worker',
      'monaco-editor/esm/vs/language/css/css.worker',
      'monaco-editor/esm/vs/language/html/html.worker',
      'monaco-editor/esm/vs/language/typescript/ts.worker',
      'monaco-editor/esm/vs/editor/editor.worker'
    ]
  }
})

