import path from "path";
import { defineConfig } from "vite";
import { viteStaticCopy } from "vite-plugin-static-copy";

export default defineConfig({
  plugins: [
    viteStaticCopy({
      targets: [
        {
          src: ["src/graphl_parser.d.ts"],
          dest: "",
          rename: "graphl-parser.d.ts",
        },
      ],
    }),
  ],
  build: {
    lib: {
      entry: path.resolve(__dirname, "src/graphl_parser.js"),
      fileName: (format) => `graphl-parser.${format}.js`,
      formats: ["es", "cjs", "umd"],
      name: "GraphlParser",
    },
  },
});
