import { createRoot } from "react-dom/client";
import App from "./App";
import { client } from "./lib/api/client.gen";

client.setConfig({
    baseURL: `${location.origin}/api`,
});

createRoot(document.getElementById("root")!).render(<App />);
