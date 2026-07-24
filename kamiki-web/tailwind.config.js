/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./assets/**/*.html",
    "./index.html"
  ],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        kamiki: {
          bg: "#0d1117",
          panel: "#161b22",
          panelHover: "#1c2333",
          border: "#30363d",
          textPrimary: "#e6edf3",
          textSecondary: "#8b949e",
          blue: "#58a6ff",
          green: "#3fb950",
          red: "#f85149",
          orange: "#d29922",
          purple: "#bc8cff",
        }
      }
    },
  },
  plugins: [],
}
