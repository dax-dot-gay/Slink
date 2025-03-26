import { defaultPlugins, defineConfig } from "@hey-api/openapi-ts";

export default defineConfig({
    input: "https://0.0.0.0:8000/openapi.json",
    output: "src/lib/api",
    plugins: [
        ...defaultPlugins,
        "@hey-api/client-axios",
        "@hey-api/transformers",
        {
            asClass: true,
            transformer: true,
            name: "@hey-api/sdk",
        },
    ],
});
