const defaultTheme = require("tailwindcss/defaultTheme");

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	theme: {
		extend: {
			spacing: {
				'content': '115ch',
			},
			backgroundColor: {
				"custom-yellow": "rgb(233, 196, 106)",
				"custom-blue": "rgb(73, 151, 222)",
				"custom-dark-gray": "#131315",
				"custom-dark-black": "#18181b",
				"custom-purple": "#6d4ee9",
				"custom-purple-dark": "#573eba"
			},
			fontFamily: {
				sans: ["Inter var", ...defaultTheme.fontFamily.sans],
			},
		},
	},
	plugins: [],
}
