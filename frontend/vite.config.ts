import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  // Relative asset URLs, so the build works from any path (GitHub Pages serves it under /<repo>/).
  base: "./",
  plugins: [react()],
  server: { host: true, port: 5173, strictPort: true },
});
