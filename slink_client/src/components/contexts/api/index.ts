import { useContext } from "react";
import { ApiProvider } from "./Provider";
import { ApiContextType, ApiState, ApiContext } from "./types";
import { IndexInfo, RedactedUser, Session } from "../../../lib/api";

export type { ApiContextType, ApiState };
export { ApiProvider };

export function useApi(): ApiContextType {
    return useContext(ApiContext);
}

export function useApiState(): ApiState {
    return useApi().state;
}

export function useSession(): Session | null {
    const api = useApi();
    if (api.state === "ready") {
        return api.clientContext.session;
    } else {
        return null;
    }
}

export function useUser(): RedactedUser | null {
    const api = useApi();
    if (api.state === "ready") {
        return api.clientContext.user ?? null;
    } else {
        return null;
    }
}

export function useApiError(): string | null {
    const api = useApi();
    if (api.state === "error") {
        return api.reason;
    } else {
        return null;
    }
}

export function useReload(): () => Promise<IndexInfo | null> {
    const api = useApi();
    if (api.state === "disconnected") {
        return async () => null;
    } else {
        return api.reload;
    }
}
