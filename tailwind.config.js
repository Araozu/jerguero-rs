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
                'serif': ["serif"],
            },
            colors: {
                "c-primary": "var(--c-primary)",
                "c-bg": "var(--c-bg)",
                "c-bg-2": "var(--c-bg-2)",
                "c-on-bg": "var(--c-on-bg)",
                "c-primary": "var(--c-primary)",
            },
        },
    },
    plugins: [
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
