import { BrowserRouter, Route, Routes } from "react-router";
import { LayoutView } from "../views/layout/Layout";
import { AuthView } from "../views/authentication/AuthView";
import { DashboardView } from "../views/dashboard/DashboardView";

export function Router() {
    return (
        <BrowserRouter>
            <Routes>
                <Route element={<LayoutView />}>
                    <Route index element={<DashboardView />} />
                    <Route path="/auth" element={<AuthView />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}
