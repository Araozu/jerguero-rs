/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.{rs,html}",
        "./static/**/*.html",
    ],
    corePlugins: {
        container: false
    },
    theme: {
        extend: {
            fontFamily: {
                'serif': ["'Playfair Display'", "serif"],
            }
        },
    },
    plugins: [
        require("daisyui"),
        function ({ addComponents }) {
            addComponents({
                '.container': {
                    maxWidth: '95%',
                    margin: "auto",
                    '@screen sm': {
                        maxWidth: '640px',
                    },
                    '@screen md': {
                        maxWidth: '768px',
                    },
                    '@screen lg': {
                        maxWidth: '1024px',
                    },
                    '@screen xl': {
                        maxWidth: '1280px',
                    },
                }
            })
        }
    ],
    daisyui: {
        themes: [
            "light",
            "dark",
            "cupcake",
            "bumblebee",
            "emerald",
            "corporate",
            "synthwave",
            "retro",
            "cyberpunk",
            "valentine",
            "halloween",
            "garden",
            "forest",
            "aqua",
            "lofi",
            "pastel",
            "fantasy",
            "wireframe",
            "black",
            "luxury",
            "dracula",
            "cmyk",
            "autumn",
            "business",
            "acid",
            "lemonade",
            "night",
            "coffee",
            "winter",
            "dim",
            "nord",
            "sunset",
        ]
    }
}
