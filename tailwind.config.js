module.exports = {
	mode: "jit",
	content: {
		files: ["src/**/*.rs", "index.html"],
	},
	darkMode: "media", // 'media' or 'class'
	theme: {
		extend: {},
	},
	variants: {
		extend: {},
	},
	safelist: ["bg-green-500"],
	plugins: [
		function({ addVariant }) {
			addVariant('child', '& > *');
			addVariant('child-hover', '& > *:hover');
		}
	],
};
