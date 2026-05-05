import js from "@eslint/js"
import nextPlugin from "@next/eslint-plugin-next"
import tseslint from "typescript-eslint"

export default tseslint.config(
  {
    ignores: [
      "out/**",
      ".next/**",
      "node_modules/**",
      "tailwind.config.js",
      "postcss.config.mjs",
    ],
  },
  js.configs.recommended,
  ...tseslint.configs.recommended,
  {
    files: ["**/*.{ts,tsx}"],
    plugins: {
      "@next/next": nextPlugin,
    },
    rules: {
      ...nextPlugin.configs.recommended.rules,
      ...nextPlugin.configs["core-web-vitals"].rules,
      "@typescript-eslint/no-explicit-any": "off",
    },
  },
)
