import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { networkInterfaces } from 'os';

const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);

function getInternalIp() {
  const nets = networkInterfaces();
  let internalIp: string;

  for (const name of Object.keys(nets)) {
    for (const net of nets[name]) {
      if (net.family === 'IPv4' && !net.internal) {
        internalIp = net.address;
        break;
      }
    }
    if (internalIp) break;
  }

  return internalIp;
}

// https://vitejs.dev/config/
export default defineConfig(async () => {
  const hmrHost = getInternalIp();
  console.log('Mobile:', mobile);
  console.log('HMR Host:', hmrHost);

  return {
    plugins: [vue()],

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
      port: 1520,
      strictPort: true,
      host: mobile ? "0.0.0.0" : false,
      hmr: mobile
        ? {
            protocol: "ws",
            host: hmrHost,
            port: 1521,
          }
        : undefined,
      watch: {
        // 3. tell vite to ignore watching `src-tauri`
        ignored: ["**/src-tauri/**"],
      },
    },
  };
});
