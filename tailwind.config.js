/** @type {import('tailwindcss').Config} */
export default {
    content: ['./index.html', './src/**/*.{html,css,svelte,js,ts}'],
    theme: {
        extend: {},
    },
    plugins: [require('daisyui')],
}
