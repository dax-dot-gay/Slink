import { ReactNode, useCallback, useEffect, useState } from "react";
import { getIndex, IndexInfo } from "../../../lib/api";
import { ApiContext } from "./types";

export function ApiProvider({
    children,
}: {
    children?: ReactNode | ReactNode[];
}) {
    const [clientContext, setClientContext] = useState<IndexInfo | null>(null);
    const [error, setError] = useState<string | null>(null);
    const reload = useCallback(async () => {
        const result = await getIndex();
        if (result.data) {
            setClientContext(result.data);
            setError(null);
            return result.data;
        } else {
            setClientContext(null);
            setError(result.message);
            return null;
        }
    }, [setClientContext, setError]);

    useEffect(() => {
        reload();
    }, [reload]);

    return (
        <ApiContext.Provider
            value={
                error
                    ? { state: "error", reason: error, reload }
                    : clientContext
                    ? { state: "ready", clientContext, reload }
                    : { state: "disconnected" }
            }
        >
            {children}
        </ApiContext.Provider>
    );
}
