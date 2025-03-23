import { createTheme } from "@mantine/core";

export const theme = createTheme({
    colors: {
        primary: [
            "#eefbf3",
            "#ddf5e5",
            "#b6eac6",
            "#8ce0a6",
            "#6ad78a",
            "#54d179",
            "#49cf6f",
            "#3ab65e",
            "#30a252",
            "#218c44",
        ],
    },
    primaryColor: "primary",
    primaryShade: {
        light: 4,
        dark: 7,
    },
    fontFamily: "Inter, sans-serif",
    fontFamilyMonospace: "Roboto Mono, monospace",
});
