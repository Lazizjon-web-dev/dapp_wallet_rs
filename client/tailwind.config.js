/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      // Example of extending the default theme
      colors: {
        'custom-blue': '#243c5a',
      },
      spacing: {
        '128': '32rem',
      },
    },
  },
  plugins: [
    // Add any Tailwind CSS plugins here, e.g., @tailwindcss/forms
    // require('@tailwindcss/forms'),
  ],
};