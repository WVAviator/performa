module.exports = {
  content: {
    files: ["./src/**/*.rs"],
    // extract: {
    //   rs: (content) => content
    //     .match(/(?<=class[:=]\(?"?)[-\w: ]+/g)
    //     ?.flatMap(s => s.split(' ')) ?? []
    // }
  },
  theme: {
    extend: {},
    fontFamily: {
      "chicago": ["pixChicago", "sans-serif"],
    },
    colors: {
      gray: "#D7D7D7",
      white: "#FEFEFE",
      magenta: "#C9BFFB",
      slate: "#3B3B3B",
      black: "#000000",
    },
    fontSize: {
      xs: "0.25rem",
      sm: "0.6rem",
      base: "0.75rem",
      xl: "1rem",
      "2xl": "1.5rem",
      "3xl": "2rem",
      "4xl": "3rem",
      "5xl": "4rem",
    }
  },
  plugins: [],
}
