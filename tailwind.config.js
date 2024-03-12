module.exports = {
  content: {
    files: ["./src/**/*.rs"],
  },
  theme: {
    extend: {},
    fontFamily: {
      "chicago": ["ChicagoSystemFont", "sans-serif"],
    },
    colors: {
      gray: "#D7D7D7",
      white: "#FEFEFE",
      magenta: "#C9BFFB",
      slate: "#3B3B3B",
      black: "#000000",
    },
    fontSize: {
      xs: "0.5rem",
      sm: "0.75rem",
      base: "0.875rem",
      xl: "1rem",
      "2xl": "1.5rem",
      "3xl": "2rem",
      "4xl": "3rem",
      "5xl": "4rem",
    }
  },
  plugins: [],
}
