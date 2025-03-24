import "@mantine/core/styles.css";
import "@mantine/notifications/styles.css";
import { MantineProvider, TypographyStylesProvider } from "@mantine/core";
import { Notifications } from "@mantine/notifications";
import { Router } from "./util/routes.tsx";
import { LocalizationProvider } from "./util/localization.tsx";
import { theme } from "./util/theme.ts";
import "./styles/index.scss";
import { getIndex } from "./lib/api/sdk.gen.ts";
import { ApiProvider } from "./components/contexts/api/Provider.tsx";

function App() {
    getIndex().then(console.log);
    return (
        <LocalizationProvider>
            <ApiProvider>
                <MantineProvider theme={theme} defaultColorScheme="dark">
                    <TypographyStylesProvider>
                        <Notifications />
                        <Router />
                    </TypographyStylesProvider>
                </MantineProvider>
            </ApiProvider>
        </LocalizationProvider>
    );
}

export default App;
