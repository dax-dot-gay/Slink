import "@mantine/core/styles.css";
import { MantineProvider, TypographyStylesProvider } from "@mantine/core";
import { Router } from "./util/routes.tsx";
import { LocalizationProvider } from "./util/localization.tsx";
import { theme } from "./util/theme.ts";
import "./styles/index.scss";

function App() {
    return (
        <LocalizationProvider>
            <MantineProvider theme={theme} defaultColorScheme="dark">
                <TypographyStylesProvider>
                    <Router />
                </TypographyStylesProvider>
            </MantineProvider>
        </LocalizationProvider>
    );
}

export default App;
