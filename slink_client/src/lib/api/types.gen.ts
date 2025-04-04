// This file is auto-generated by @hey-api/openapi-ts

export type IndexInfo = {
    session: Session;
    runner_mode: RunnerMode;
    user?: RedactedUser | null;
};

export type Session = {
    _id?: string;
    created: Date;
    last_connection: Date;
    user?: TsLink | null;
};

export type TsLink = {
    collection: string;
    id: string;
};

export type RunnerMode = 'docker_host';

export type RedactedUser = {
    id: string;
    username: string;
    superuser: boolean;
};

export type LoginModel = {
    username: string;
    password: string;
};

export type MinecraftServer = {
    _id?: string;
    name: string;
    owner: TsLink;
    minecraft_version: MinecraftVersionMetadata;
    modloader_version?: ServerBinaryVersion | null;
};

export type MinecraftVersionMetadata = {
    client: MinecraftFileDownload;
    server: MinecraftFileDownload;
    java_version: JavaVersion;
    version: MinecraftVersion;
};

export type MinecraftFileDownload = {
    url: string;
    sha1: string;
    size: number;
};

export type JavaVersion = number;

export type MinecraftVersion = {
    id: string;
    type: MinecraftVersionType;
    url: string;
    time: Date;
    releaseTime: Date;
    sha1: string;
    complianceLevel: number;
};

export type MinecraftVersionType = 'release' | 'snapshot';

export type ServerBinaryVersion = {
    Fabric: FabricServerBinaryVersion;
};

export type FabricServerBinaryVersion = {
    component: 'loader';
    version: string;
    stable: boolean;
} | {
    component: 'installer';
    version: string;
    stable: boolean;
};

export type ServerCreationParams = {
    name: string;
    minecraft_version: string;
    mod_loader?: ServerBinaryVersion | null;
};

export type MinecraftVersionList = {
    latest: MinecraftVersionLatest;
    versions: Array<MinecraftVersion>;
};

export type MinecraftVersionLatest = {
    release: string;
    snapshot: string;
};

export type GetIndexData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/';
};

export type GetIndexResponses = {
    200: IndexInfo;
};

export type GetIndexResponse = GetIndexResponses[keyof GetIndexResponses];

export type LogoutData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/auth/login';
};

export type LogoutErrors = {
    /**
     * # [Error 400](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
     * An error occurred while trying to parse the user's request.
     */
    400: unknown;
    /**
     * # [Error 401](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)
     * User is not authorized to perform this request.
     */
    401: unknown;
    /**
     * # [Error 404](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)
     * Requested resource not found
     */
    404: unknown;
    /**
     * # [Error 500](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)
     * Internal server error occurred while processing request.
     */
    500: unknown;
};

export type LogoutResponses = {
    200: unknown;
};

export type LoginData = {
    body: LoginModel;
    path?: never;
    query?: never;
    url: '/auth/login';
};

export type LoginErrors = {
    /**
     * # [Error 400](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
     * An error occurred while trying to parse the user's request.
     */
    400: unknown;
    /**
     * # [Error 401](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)
     * User is not authorized to perform this request.
     */
    401: unknown;
    /**
     * # [Error 404](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)
     * Requested resource not found
     */
    404: unknown;
    /**
     * # [Error 500](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)
     * Internal server error occurred while processing request.
     */
    500: unknown;
};

export type LoginResponses = {
    200: RedactedUser;
};

export type LoginResponse = LoginResponses[keyof LoginResponses];

export type GetOwnedServersData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/servers/owned';
};

export type GetOwnedServersErrors = {
    /**
     * # [Error 400](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
     * An error occurred while trying to parse the user's request.
     */
    400: unknown;
    /**
     * # [Error 401](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)
     * User is not authorized to perform this request.
     */
    401: unknown;
    /**
     * # [Error 404](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)
     * Requested resource not found
     */
    404: unknown;
    /**
     * # [Error 500](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)
     * Internal server error occurred while processing request.
     */
    500: unknown;
};

export type GetOwnedServersResponses = {
    200: Array<MinecraftServer>;
};

export type GetOwnedServersResponse = GetOwnedServersResponses[keyof GetOwnedServersResponses];

export type CreateServerData = {
    body: ServerCreationParams;
    path?: never;
    query?: never;
    url: '/servers/create';
};

export type CreateServerErrors = {
    /**
     * # [Error 400](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
     * An error occurred while trying to parse the user's request.
     */
    400: unknown;
    /**
     * # [Error 401](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)
     * User is not authorized to perform this request.
     */
    401: unknown;
    /**
     * # [Error 404](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)
     * Requested resource not found
     */
    404: unknown;
    /**
     * # [Error 500](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)
     * Internal server error occurred while processing request.
     */
    500: unknown;
};

export type CreateServerResponses = {
    200: MinecraftServer;
};

export type CreateServerResponse = CreateServerResponses[keyof CreateServerResponses];

export type ListMinecraftVersionsData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/providers/minecraft/versions';
};

export type ListMinecraftVersionsErrors = {
    /**
     * # [Error 400](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
     * An error occurred while trying to parse the user's request.
     */
    400: unknown;
    /**
     * # [Error 401](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)
     * User is not authorized to perform this request.
     */
    401: unknown;
    /**
     * # [Error 404](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)
     * Requested resource not found
     */
    404: unknown;
    /**
     * # [Error 500](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)
     * Internal server error occurred while processing request.
     */
    500: unknown;
};

export type ListMinecraftVersionsResponses = {
    200: MinecraftVersionList;
};

export type ListMinecraftVersionsResponse = ListMinecraftVersionsResponses[keyof ListMinecraftVersionsResponses];

export type GetLatestReleaseVersionData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/providers/minecraft/versions/latest_release';
};

export type GetLatestReleaseVersionErrors = {
    /**
     * # [Error 400](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
     * An error occurred while trying to parse the user's request.
     */
    400: unknown;
    /**
     * # [Error 401](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)
     * User is not authorized to perform this request.
     */
    401: unknown;
    /**
     * # [Error 404](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)
     * Requested resource not found
     */
    404: unknown;
    /**
     * # [Error 500](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)
     * Internal server error occurred while processing request.
     */
    500: unknown;
};

export type GetLatestReleaseVersionResponses = {
    200: MinecraftVersion;
};

export type GetLatestReleaseVersionResponse = GetLatestReleaseVersionResponses[keyof GetLatestReleaseVersionResponses];

export type GetLatestSnapshotVersionData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/providers/minecraft/versions/latest_snapshot';
};

export type GetLatestSnapshotVersionErrors = {
    /**
     * # [Error 400](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
     * An error occurred while trying to parse the user's request.
     */
    400: unknown;
    /**
     * # [Error 401](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)
     * User is not authorized to perform this request.
     */
    401: unknown;
    /**
     * # [Error 404](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)
     * Requested resource not found
     */
    404: unknown;
    /**
     * # [Error 500](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)
     * Internal server error occurred while processing request.
     */
    500: unknown;
};

export type GetLatestSnapshotVersionResponses = {
    200: MinecraftVersion;
};

export type GetLatestSnapshotVersionResponse = GetLatestSnapshotVersionResponses[keyof GetLatestSnapshotVersionResponses];

export type GetSpecificVersionData = {
    body?: never;
    path: {
        id: string;
    };
    query?: never;
    url: '/providers/minecraft/versions/{id}';
};

export type GetSpecificVersionErrors = {
    /**
     * # [Error 400](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
     * An error occurred while trying to parse the user's request.
     */
    400: unknown;
    /**
     * # [Error 401](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)
     * User is not authorized to perform this request.
     */
    401: unknown;
    /**
     * # [Error 404](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)
     * Requested resource not found
     */
    404: unknown;
    /**
     * # [Error 500](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)
     * Internal server error occurred while processing request.
     */
    500: unknown;
};

export type GetSpecificVersionResponses = {
    200: MinecraftVersion;
};

export type GetSpecificVersionResponse = GetSpecificVersionResponses[keyof GetSpecificVersionResponses];

export type GetVersionMetadataData = {
    body?: never;
    path: {
        id: string;
    };
    query?: never;
    url: '/providers/minecraft/versions/{id}/metadata';
};

export type GetVersionMetadataErrors = {
    /**
     * # [Error 400](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
     * An error occurred while trying to parse the user's request.
     */
    400: unknown;
    /**
     * # [Error 401](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)
     * User is not authorized to perform this request.
     */
    401: unknown;
    /**
     * # [Error 404](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)
     * Requested resource not found
     */
    404: unknown;
    /**
     * # [Error 500](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)
     * Internal server error occurred while processing request.
     */
    500: unknown;
};

export type GetVersionMetadataResponses = {
    200: MinecraftVersionMetadata;
};

export type GetVersionMetadataResponse = GetVersionMetadataResponses[keyof GetVersionMetadataResponses];

export type GetProviderComponentsData = {
    body?: never;
    path: {
        name: string;
    };
    query?: never;
    url: '/providers/server_binary/{name}/components';
};

export type GetProviderComponentsErrors = {
    /**
     * # [Error 400](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
     * An error occurred while trying to parse the user's request.
     */
    400: unknown;
    /**
     * # [Error 401](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)
     * User is not authorized to perform this request.
     */
    401: unknown;
    /**
     * # [Error 404](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)
     * Requested resource not found
     */
    404: unknown;
    /**
     * # [Error 500](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)
     * Internal server error occurred while processing request.
     */
    500: unknown;
};

export type GetProviderComponentsResponses = {
    200: Array<string>;
};

export type GetProviderComponentsResponse = GetProviderComponentsResponses[keyof GetProviderComponentsResponses];

export type GetCompatibleVersionsData = {
    body?: never;
    path: {
        name: string;
        minecraft: string;
    };
    query?: never;
    url: '/providers/server_binary/{name}/{minecraft}/components';
};

export type GetCompatibleVersionsErrors = {
    /**
     * # [Error 400](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400)
     * An error occurred while trying to parse the user's request.
     */
    400: unknown;
    /**
     * # [Error 401](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401)
     * User is not authorized to perform this request.
     */
    401: unknown;
    /**
     * # [Error 404](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404)
     * Requested resource not found
     */
    404: unknown;
    /**
     * # [Error 500](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500)
     * Internal server error occurred while processing request.
     */
    500: unknown;
};

export type GetCompatibleVersionsResponses = {
    200: {
        [key: string]: Array<ServerBinaryVersion>;
    };
};

export type GetCompatibleVersionsResponse = GetCompatibleVersionsResponses[keyof GetCompatibleVersionsResponses];

export type ClientOptions = {
    baseURL: 'https://0.0.0.0:8000' | (string & {});
};