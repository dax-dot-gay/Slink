import { BrowserRouter, Route, Routes } from "react-router";
import { LayoutView } from "../views/layout/Layout";

export function Router() {
  return (
    <BrowserRouter>
      <Routes>
        <Route index element={<LayoutView />} />
      </Routes>
    </BrowserRouter>
  );
}
