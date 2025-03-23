import {
    ActionIcon,
    AppShell,
    Avatar,
    Box,
    Burger,
    Divider,
    Group,
    Paper,
    Skeleton,
    Stack,
    Text,
    useMantineTheme,
} from "@mantine/core";
import { useTranslation } from "react-i18next";
import {
    TbChevronsLeft,
    TbCube,
    TbLogout2,
    TbSettings,
    TbUser,
} from "react-icons/tb";
import { useDisclosure, useMediaQuery } from "@mantine/hooks";
import { Outlet } from "react-router";

export function LayoutView() {
    const { t } = useTranslation();
    const theme = useMantineTheme();
    const isMobile = useMediaQuery(
        `(max-width: ${theme.breakpoints.sm})`,
        false
    );
    const [collapsed, { toggle }] = useDisclosure(false);
    return (
        <AppShell
            layout="alt"
            navbar={{
                width: 312,
                breakpoint: "sm",
                collapsed: { desktop: false, mobile: collapsed },
            }}
            header={{ height: 72 }}
        >
            <AppShell.Header hiddenFrom="sm">
                <Group gap="sm" h="100%" pl="sm">
                    <Burger opened={!collapsed} onClick={toggle} />
                    <Divider orientation="vertical" />
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
                    <Stack gap="sm" p="sm" style={{ flexGrow: 1 }}></Stack>
                    <Divider />
                    <Group
                        gap="sm"
                        p="sm"
                        justify="space-between"
                        wrap="nowrap"
                    >
                        <Group gap="sm" wrap="nowrap" style={{ flexGrow: 1 }}>
                            <Avatar>
                                <TbUser size={20} />
                            </Avatar>
                            <Skeleton
                                animate={false}
                                style={{ flexGrow: 1 }}
                                h="32px"
                            />
                        </Group>
                        <Group gap="xs">
                            <ActionIcon radius="sm" variant="light" size="lg">
                                <TbSettings />
                            </ActionIcon>
                            <ActionIcon radius="sm" variant="light" size="lg">
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
