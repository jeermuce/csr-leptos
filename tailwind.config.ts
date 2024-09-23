import type { Config } from "tailwindcss";
import type { PluginAPI } from "tailwindcss/types/config";
import { fontFamily } from "tailwindcss/defaultTheme";

const config: Config = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [
    
    function ({ addVariant }: PluginAPI) {
      addVariant("child", "& > *");
      addVariant("child-hover", "& > *:hover");
      addVariant("parent", ":not(:empty) > &");
      addVariant("parent-hover", ":not(:empty) > &:hover");
      addVariant("descendants", "& *");
      addVariant("sibling", "& ~ &");
      addVariant("sibling-hover", "& ~ &:hover");
      
    },
  ],
};

export default config;
