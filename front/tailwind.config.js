/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			fontFamily: {
				noto: ['Noto Sans']
			}
		}
	},
	plugins: [require('daisyui')],
	daisyui: {
		themes: ['dark']
	}
};
