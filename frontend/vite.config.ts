import { defineConfig } from 'vite'

export default defineConfig({
	build: {
		outDir: 'dist',
		emptyOutDir: true,
	},
  server: {
    proxy: {
      '/__stub': 'http://127.0.0.1:8080'
    }
  }
})
