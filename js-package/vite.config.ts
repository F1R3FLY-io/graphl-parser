import path from "path";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import { viteStaticCopy } from "vite-plugin-static-copy";

export default defineConfig({
  plugins: [
    wasm(),
    viteStaticCopy({
      targets: [
        {
          src: ["src/*.wasm", "src/*.d.ts"],
          dest: "",
        },
      ],
    }),
  ],
  build: {
    lib: {
      entry: path.resolve(__dirname, "src/graph_to_rholang_parser.js"),
      fileName: (format) => `graphl-parser.${format}.js`,
      formats: ["es"],
      name: "GraphlParser",
    },
  },
});
