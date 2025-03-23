import "@mantine/core/styles.css";
import {
  AppShell,
  MantineProvider,
  TypographyStylesProvider,
} from "@mantine/core";
import { shadcnCssVariableResolver } from "./util/theme/cssVariableResolver.ts";
import { shadcnTheme } from "./util/theme/theme.ts";
import "./util/theme/style.css";

function App() {
  return (
    <MantineProvider
      theme={shadcnTheme}
      cssVariablesResolver={shadcnCssVariableResolver}
      defaultColorScheme="dark"
    >
      <TypographyStylesProvider>
        <AppShell>TEST</AppShell>
      </TypographyStylesProvider>
    </MantineProvider>
  );
}

export default App;
