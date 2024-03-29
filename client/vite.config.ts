import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/game': {
        target: 'http://127.0.0.1:8080',
        ws: true,
      },
      '/user': {
        target: 'http://127.0.0.1:8080',
      },
      '/login': {
        target: 'http://127.0.0.1:8080',
      },
    },
  },
});
