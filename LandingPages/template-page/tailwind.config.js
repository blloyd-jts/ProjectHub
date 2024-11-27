module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
    },
  },
  theme: {
    extend: {
      colors: {
        customBlue: '#4A6574',
        customOrange: '#DC9347',
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}

