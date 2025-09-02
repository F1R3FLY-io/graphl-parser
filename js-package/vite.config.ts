import path from "path";
import { defineConfig } from "vite";
import { viteStaticCopy } from "vite-plugin-static-copy";

export default defineConfig({
  esbuild: { target: "esnext" },
  plugins: [
    viteStaticCopy({
      targets: [
        { src: ["src/*.d.ts"], dest: "" },
      ],
    }),
  ],
  build: {
    target: "esnext",
    lib: {
      entry: path.resolve(__dirname, "js-wrapper/index.js"),
      fileName: (format) => `graphl-parser.${format}.js`,
      formats: ["es", "cjs"],
      name: "GraphlParser",
    },
  },
});
