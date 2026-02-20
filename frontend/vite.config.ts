import * as fs from "node:fs";
import * as os from "node:os";
import path from "node:path";
import { fileURLToPath, URL } from "node:url";
import { defineConfig, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";
import vueDevTools from "vite-plugin-vue-devtools";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, path.resolve(__dirname, "../"), "VITE_");
  let certDir = process.env.CERT_DIR || env.VITE_CERT_DIR || "~/.dev-certs/";
  if (certDir.startsWith("~")) {
    certDir = path.join(os.homedir(), certDir.slice(1));
  }

  return {
    build: {
      emptyOutDir: true,
      outDir: '../dist',
    },
    envDir: "../",
    envPrefix: "VITE_",
    plugins: [
      vue(),
      vueDevTools(),
    ],
    resolve: {
      alias: {
        "@": fileURLToPath(new URL("./src", import.meta.url))
      },
    },
    server: {
      https: {
        cert: fs.readFileSync(path.join(certDir, "cert.pem")),
        key: fs.readFileSync(path.join(certDir, "key.pem")),
      },
      port: 5173,
    },
  };
});
