/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        quickSand: ['Quicksand', 'sans-serif'],
      },
      animation: {
        'gradient-x': 'gradient-x 1.5s linear infinite',
      },
      keyframes: {
        'gradient-x': {
          '0%': { transform: 'translateX(-100%)' },
          '100%': { transform: 'translateX(100%)' },
        },
      }
    },
  },
  plugins: [],
}