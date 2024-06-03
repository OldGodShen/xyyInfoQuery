import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { internalIpV4 } from "internal-ip";

// @ts-expect-error process is a nodejs global
const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);

async function getInternalIp() {
  try {
    const ip = await internalIpV4();
    if (!ip) {
      throw new Error("Internal IP is undefined");
    }
    return ip;
  } catch (error) {
    console.error("Failed to get internal IP:", error);
    // Fallback to a default IP or handle the error as needed
    return "172.17.0.1";
  }
}

// https://vitejs.dev/config/
export default defineConfig(async () => {
  const hmrHost = await getInternalIp();
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
