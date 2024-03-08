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
  },
  plugins: [],
}
