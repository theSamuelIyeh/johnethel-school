/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    '../src/views/**/*.{rs,js,html}',
    'node_modules/preline/dist/*.js',
  ],
  darkMode: 'class',
  theme: {
    fontFamily: {
      sans: ["Poppins", "sans-serif"],
    },
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('preline/plugin'),
  ],
}

