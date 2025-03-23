import { ReactNode } from "react";
import { createInstance } from "i18next";
import { I18nextProvider } from "react-i18next";
import * as LangEn from "../lang/en.json";

const i18n = createInstance({
    fallbackLng: "en",
    interpolation: {
        escapeValue: false,
    },
    resources: {
        en: {
            translation: LangEn,
        },
    },
});

i18n.init();

export function LocalizationProvider({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    return (
        <I18nextProvider i18n={i18n} defaultNS={"translation"}>
            {children}
        </I18nextProvider>
    );
}
