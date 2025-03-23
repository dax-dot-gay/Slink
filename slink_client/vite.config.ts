import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import { readFileSync } from "fs";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    host: "0.0.0.0",
    https: {
      key: readFileSync("/slink/certificates/client/slink.key"),
      cert: readFileSync("/slink/certificates/client/slink.crt"),
    },
    proxy: {
      "/api": {
        target: "https://0.0.0.0:8000",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ""),
        secure: false,
      },
    },
  },
});
