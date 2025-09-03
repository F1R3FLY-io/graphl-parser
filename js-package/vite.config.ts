import path from "path";
import { defineConfig } from "vite";
import { viteStaticCopy } from "vite-plugin-static-copy";

export default defineConfig({
  plugins: [
    viteStaticCopy({
      targets: [
        {
          src: ["src/graph_to_rholang_parser.d.ts"],
          dest: "",
          rename: "graphl-parser.d.ts",
        },
      ],
    }),
  ],
  build: {
    lib: {
      entry: path.resolve(__dirname, "src/graph_to_rholang_parser.js"),
      fileName: (format) => `graphl-parser.${format}.js`,
      formats: ["es", "cjs", "umd"],
      name: "GraphlParser",
    },
  },
});
