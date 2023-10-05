const defaultTheme = require("tailwindcss/defaultTheme");

/** @type {import("tailwindcss").Config} */
module.exports = {
    content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
    theme: {
        extend: {
            backgroundImage: {
                "patten-primary": "radial-gradient(rgb(var(--md-sys-color-primary-container) / 1) 0.5px, rgb(var(--md-sys-color-background) / 0.6) 0.5px)",
                "patten-surface": "radial-gradient(rgb(var(--md-sys-color-surface-container) / 1) 0.5px, rgb(var(--md-sys-color-background) / 0.6) 0.5px)",
            },
            boxShadow: {
                1: [
                    "0 1px 2px rgba(0, 0, 0, 0.3)",
                    "0 1px 3px 1px rgba(0, 0, 0, 0.15)",
                ],
                2: [
                    "0 1px 2px rgba(0, 0, 0, 0.3)",
                    "0 2px 3px 1px rgba(0, 0, 0, 0.15)",
                ],
                3: [
                    "0 1px 3px rgba(0, 0, 0, 0.3)",
                    "0 4px 8px 3px rgba(0, 0, 0, 0.15)",
                ],
                4: [
                    "0 2px 3px rgba(0, 0, 0, 0.3)",
                    "0 6px 10px 4px rgba(0, 0, 0, 0.15)",
                ],
                5: [
                    "0 4px 4px rgba(0, 0, 0, 0.3)",
                    "0 8px 12px 6px rgba(0, 0, 0, 0.15)",
                ],
            },
            colors: {
                primary: "rgb(var(--md-sys-color-primary) / <alpha-value>)",
                onPrimary: "rgb(var(--md-sys-color-on-primary) / <alpha-value>)",
                primaryContainer:
                    "rgb(var(--md-sys-color-primary-container) / <alpha-value>)",
                onPrimaryContainer:
                    "rgb(var(--md-sys-color-on-primary-container) / <alpha-value>)",
                primaryFixed: "rgb(var(--md-sys-color-primary-fixed) / <alpha-value>)",
                onPrimaryFixed:
                    "rgb(var(--md-sys-color-on-primary-fixed) / <alpha-value>)",
                primaryFixedDim:
                    "rgb(var(--md-sys-color-primary-fixed-dim) / <alpha-value>)",
                onPrimaryFixedVariant:
                    "rgb(var(--md-sys-color-on-primary-fixed-variant) / <alpha-value>)",
                secondary: "rgb(var(--md-sys-color-secondary) / <alpha-value>)",
                onSecondary: "rgb(var(--md-sys-color-on-secondary) / <alpha-value>)",
                secondaryContainer:
                    "rgb(var(--md-sys-color-secondary-container) / <alpha-value>)",
                onSecondaryContainer:
                    "rgb(var(--md-sys-color-on-secondary-container) / <alpha-value>)",
                secondaryFixed:
                    "rgb(var(--md-sys-color-secondary-fixed) / <alpha-value>)",
                onSecondaryFixed:
                    "rgb(var(--md-sys-color-on-secondary-fixed) / <alpha-value>)",
                secondaryFixedDim:
                    "rgb(var(--md-sys-color-secondary-fixed-dim) / <alpha-value>)",
                onSecondaryFixedVariant:
                    "rgb(var(--md-sys-color-on-secondary-fixed-variant) / <alpha-value>)",
                tertiary: "rgb(var(--md-sys-color-tertiary) / <alpha-value>)",
                onTertiary: "rgb(var(--md-sys-color-on-tertiary) / <alpha-value>)",
                tertiaryContainer:
                    "rgb(var(--md-sys-color-tertiary-container) / <alpha-value>)",
                onTertiaryContainer:
                    "rgb(var(--md-sys-color-on-tertiary-container) / <alpha-value>)",
                tertiaryFixed:
                    "rgb(var(--md-sys-color-tertiary-fixed) / <alpha-value>)",
                onTertiaryFixed:
                    "rgb(var(--md-sys-color-on-tertiary-fixed) / <alpha-value>)",
                tertiaryFixedDim:
                    "rgb(var(--md-sys-color-tertiary-fixed-dim) / <alpha-value>)",
                onTertiaryFixedVariant:
                    "rgb(var(--md-sys-color-on-tertiary-fixed-variant) / <alpha-value>)",
                error: "rgb(var(--md-sys-color-error) / <alpha-value>)",
                onError: "rgb(var(--md-sys-color-on-error) / <alpha-value>)",
                errorContainer:
                    "rgb(var(--md-sys-color-error-container) / <alpha-value>)",
                onErrorContainer:
                    "rgb(var(--md-sys-color-on-error-container) / <alpha-value>)",
                outline: "rgb(var(--md-sys-color-outline) / <alpha-value>)",
                background: "rgb(var(--md-sys-color-background) / <alpha-value>)",
                onBackground: "rgb(var(--md-sys-color-on-background) / <alpha-value>)",
                surface: "rgb(var(--md-sys-color-surface) / <alpha-value>)",
                onSurface: "rgb(var(--md-sys-color-on-surface) / <alpha-value>)",
                surfaceVariant:
                    "rgb(var(--md-sys-color-surface-variant) / <alpha-value>)",
                onSurfaceVariant:
                    "rgb(var(--md-sys-color-on-surface-variant) / <alpha-value>)",
                inverseSurface:
                    "rgb(var(--md-sys-color-inverse-surface) / <alpha-value>)",
                inverseOnSurface:
                    "rgb(var(--md-sys-color-inverse-on-surface) / <alpha-value>)",
                inversePrimary:
                    "rgb(var(--md-sys-color-inverse-primary) / <alpha-value>)",
                shadow: "rgb(var(--md-sys-color-shadow) / <alpha-value>)",
                surfaceTint: "rgb(var(--md-sys-color-surface-tint) / <alpha-value>)",
                outlineVariant:
                    "rgb(var(--md-sys-color-outline-variant) / <alpha-value>)",
                scrim: "rgb(var(--md-sys-color-scrim) / <alpha-value>)",
                surfaceContainerHighest:
                    "rgb(var(--md-sys-color-surface-container-highest) / <alpha-value>)",
                surfaceContainerHigh:
                    "rgb(var(--md-sys-color-surface-container-high) / <alpha-value>)",
                surfaceContainer:
                    "rgb(var(--md-sys-color-surface-container) / <alpha-value>)",
                surfaceContainerLow:
                    "rgb(var(--md-sys-color-surface-container-low) / <alpha-value>)",
                surfaceContainerLowest:
                    "rgb(var(--md-sys-color-surface-container-lowest) / <alpha-value>)",
                surfaceBright:
                    "rgb(var(--md-sys-color-surface-bright) / <alpha-value>)",
                surfaceDim: "rgb(var(--md-sys-color-surface-dim) / <alpha-value>)",
            },
            fontFamily: {
                sans: [
                    // "Roboto",
                    // "Noto Sans SC",
                    // "Segoe UI Emoji",
                    ...defaultTheme.fontFamily.sans,
                ],
                serif: [
                    // "KpRoman",
                    // "Noto Serif SC",
                    ...defaultTheme.fontFamily.serif,
                    // "Segoe UI Emoji",
                ],
                emj: ["Segoe UI Emoji"],
            },
        },
    },
    plugins: [],
};
