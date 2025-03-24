import {
    Button,
    Collapse,
    ActionIcon,
    AppShell,
    Avatar,
    Box,
    Burger,
    Divider,
    Group,
    Paper,
    Stack,
    Text,
    useMantineTheme,
    Space,
} from "@mantine/core";
import { useTranslation } from "react-i18next";
import {
    TbAffiliate,
    TbChevronDown,
    TbChevronRight,
    TbChevronsLeft,
    TbCube,
    TbLogout2,
    TbServer,
    TbSettings,
    TbShield,
    TbUser,
    TbX,
} from "react-icons/tb";
import { useDisclosure, useMediaQuery } from "@mantine/hooks";
import { Outlet, useLocation, useNavigate } from "react-router";
import { useApiState, useReload, useUser } from "../../components/contexts/api";
import { useEffect } from "react";
import { logout } from "../../lib/api";

export function LayoutView() {
    const { t } = useTranslation();
    const theme = useMantineTheme();
    const isMobile = useMediaQuery(
        `(max-width: ${theme.breakpoints.sm})`,
        false
    );
    const [collapsed, { toggle }] = useDisclosure(false);
    const nav = useNavigate();
    const location = useLocation();
    const user = useUser();
    const apiState = useApiState();
    const reload = useReload();

    const [servers, { toggle: toggleServers }] = useDisclosure(true);
    const [proxies, { toggle: toggleProxies }] = useDisclosure(true);

    useEffect(() => {
        if (
            apiState === "ready" &&
            user === null &&
            location.pathname !== "/auth"
        ) {
            nav("/auth");
        }
    }, [apiState, user?.id, location.pathname]);

    return (
        <AppShell
            layout="alt"
            navbar={{
                width: 312,
                breakpoint: "sm",
                collapsed: {
                    desktop: location.pathname === "/auth" ? true : false,
                    mobile: location.pathname === "/auth" ? true : collapsed,
                },
            }}
            header={{ height: 72 }}
        >
            <AppShell.Header hiddenFrom="sm">
                <Group gap="sm" h="100%" pl="sm">
                    {location.pathname !== "/auth" && (
                        <>
                            <Burger opened={!collapsed} onClick={toggle} />
                            <Divider orientation="vertical" />
                        </>
                    )}
                    <TbCube size={24} />
                    <Text size="xl" m={0}>
                        {t("lex.appName")}
                    </Text>
                </Group>
            </AppShell.Header>
            <AppShell.Navbar p={0}>
                <Stack gap={0} h="100%">
                    <Group gap="sm" wrap="nowrap" p="sm">
                        <Paper
                            p="sm"
                            px="md"
                            radius="sm"
                            className="paper-light"
                            style={{ flexGrow: 1 }}
                        >
                            <Group
                                gap="sm"
                                justify="space-between"
                                wrap="nowrap"
                            >
                                <TbCube size={32} />
                                <Stack gap={0} align="end">
                                    <Text size="xl" m={0}>
                                        {t("lex.appName")}
                                    </Text>
                                    <Text size="xs" c="dimmed">
                                        {t("lex.appSub")}
                                    </Text>
                                </Stack>
                            </Group>
                        </Paper>
                        <ActionIcon
                            radius="sm"
                            h="100%"
                            w={48}
                            hiddenFrom="sm"
                            variant="light"
                            onClick={toggle}
                        >
                            <TbChevronsLeft />
                        </ActionIcon>
                    </Group>
                    <Stack gap={0} p="sm" style={{ flexGrow: 1 }}>
                        <Button
                            variant={servers ? "light" : "subtle"}
                            leftSection={<TbServer size={20} />}
                            rightSection={
                                servers ? <TbChevronDown /> : <TbChevronRight />
                            }
                            justify="start"
                            className="nav-section-button"
                            onClick={toggleServers}
                            size="md"
                        >
                            {t("nav.servers")}
                        </Button>
                        <Collapse in={servers} pt="sm">
                            <Stack
                                gap="sm"
                                p={0}
                                pl="xs"
                                ml="xs"
                                style={{
                                    borderLeft:
                                        "solid 1px var(--mantine-color-default-border)",
                                }}
                            ></Stack>
                        </Collapse>
                        <Space h="sm" />
                        <Button
                            variant={proxies ? "light" : "subtle"}
                            leftSection={<TbAffiliate size={20} />}
                            rightSection={
                                proxies ? <TbChevronDown /> : <TbChevronRight />
                            }
                            justify="start"
                            className="nav-section-button"
                            onClick={toggleProxies}
                            size="md"
                        >
                            {t("nav.proxies")}
                        </Button>
                        <Collapse in={proxies} pt="sm">
                            <Stack
                                gap="sm"
                                p={0}
                                pl="xs"
                                ml="xs"
                                style={{
                                    borderLeft:
                                        "solid 1px var(--mantine-color-default-border)",
                                }}
                            >
                                <Paper className="paper-light" p="xs">
                                    <Group gap="sm" justify="space-between">
                                        <TbX size={20} />
                                        <Text c="dimmed" size="sm">
                                            {t("nav.noProxies")}
                                        </Text>
                                    </Group>
                                </Paper>
                            </Stack>
                        </Collapse>
                    </Stack>
                    <Divider />
                    <Group
                        gap="sm"
                        p="sm"
                        justify="space-between"
                        wrap="nowrap"
                    >
                        <Group gap="sm" wrap="nowrap" style={{ flexGrow: 1 }}>
                            <Avatar>
                                {user?.superuser ? (
                                    <TbShield size={20} />
                                ) : (
                                    <TbUser size={20} />
                                )}
                            </Avatar>
                            <Text>{user?.username}</Text>
                        </Group>
                        <Group gap="xs">
                            <ActionIcon radius="sm" variant="light" size="lg">
                                <TbSettings />
                            </ActionIcon>
                            <ActionIcon
                                radius="sm"
                                variant="light"
                                size="lg"
                                onClick={() => {
                                    logout().then(() => reload());
                                }}
                            >
                                <TbLogout2 />
                            </ActionIcon>
                        </Group>
                    </Group>
                </Stack>
            </AppShell.Navbar>
            <AppShell.Main
                className="layout-root"
                pt={isMobile ? undefined : "0px"}
            >
                <Box
                    className="layout-wrapper"
                    h={isMobile ? "calc(100vh - 72px)" : "100vh"}
                >
                    <Outlet />
                </Box>
            </AppShell.Main>
        </AppShell>
    );
}
