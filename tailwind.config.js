/** @type {import('tailwindcss').Config} */
export const content = ["./src/**/*.rs"];
export const theme = {
    extend: {
        colors: {
            gruvbox: {
                // Light theme
                'bg-h': '#f9f5d7',
                'bg': '#fbf1c7',
                'bg-s': '#f2e5bc',
                'fg': '#3c3836',
                'gray-l': '#7c6f64',
                'gray': '#665c54',

                // Dark theme
                'bg-h-dark': '#1d2021',
                'bg-dark': '#282828',
                'bg-s-dark': '#32302f',
                'fg-dark': '#ebdbb2',
                'gray-l-dark': '#a89984',
                'gray-dark': '#928374',

                // Colors
                'green': '#98971a',
                'green-bright': '#b8bb26',
                'orange': '#d65d0e',
                'orange-bright': '#fe8019',
                'yellow': '#b57614',
                'yellow-bright': '#fabd2f',

                // Borders
                'border-light': '#d5c4a1',
                'border-dark': '#504945',
            },
        },
        typography: ({ theme }) => ({
            DEFAULT: {
                css: {
                    '--tw-prose-body': theme('colors.gruvbox.fg'),
                    '--tw-prose-headings': theme('colors.gruvbox.fg'),
                    '--tw-prose-links': theme('colors.gruvbox.green'),
                    '--tw-prose-bold': theme('colors.gruvbox.orange'),
                    '--tw-prose-code': theme('colors.gruvbox.yellow'),
                    '--tw-prose-quotes': theme('colors.gruvbox.gray'),
                    '--tw-prose-quote-borders': theme('colors.gruvbox.green'),
                },
            },
            invert: {
                css: {
                    '--tw-prose-body': theme('colors.gruvbox.fg-dark'),
                    '--tw-prose-headings': theme('colors.gruvbox.fg-dark'),
                    '--tw-prose-links': theme('colors.gruvbox.green-bright'),
                    '--tw-prose-bold': theme('colors.gruvbox.orange-bright'),
                    '--tw-prose-code': theme('colors.gruvbox.yellow-bright'),
                    '--tw-prose-quotes': theme('colors.gruvbox.gray-l-dark'),
                    '--tw-prose-quote-borders': theme('colors.gruvbox.green-bright'),
                },
            },
        }),
    },
};
export const plugins = [
    require('@tailwindcss/typography'),
];
export const darkMode = 'media';
