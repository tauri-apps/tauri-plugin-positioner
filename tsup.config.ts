import { defineConfig } from "tsup";

export default defineConfig({
    entry: ["client/index.ts"],
    format: ["esm"],
    outDir: "client-dist",
    clean: true,
    dts: true
});
