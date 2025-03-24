import { createContext } from "react";
import { IndexInfo } from "../../../lib/api";

export type ApiContextType =
    | {
          state: "disconnected";
      }
    | {
          state: "ready";
          clientContext: IndexInfo;
          reload: () => Promise<IndexInfo | null>;
      }
    | {
          state: "error";
          reason: string;
          reload: () => Promise<IndexInfo | null>;
      };

export type ApiState = ApiContextType["state"];
export const ApiContext = createContext<ApiContextType>({
    state: "error",
    reason: "Wrapper context has not been initialized.",
    reload: async () => null,
});
