import {
    Button,
    Center,
    Divider,
    Group,
    Paper,
    Stack,
    Text,
    TextInput,
    Title,
} from "@mantine/core";
import { useTranslation } from "react-i18next";
import { useApiState, useReload, useUser } from "../../components/contexts/api";
import { useNavigate } from "react-router";
import { useEffect } from "react";
import { TbCube, TbLock, TbLogin2, TbUser } from "react-icons/tb";
import { useForm } from "@mantine/form";
import { PasswordField } from "../../components/fields/PasswordField";
import { AuthenticationService } from "../../lib/api";
import { useNotifications } from "../../util/notifs";

export function AuthView() {
    const { t } = useTranslation();
    const user = useUser();
    const apiState = useApiState();
    const nav = useNavigate();
    const reload = useReload();
    const { error } = useNotifications();

    useEffect(() => {
        if (apiState === "ready" && user !== null) {
            nav("/");
        }
    }, [user?.id, apiState]);

    const form = useForm({
        initialValues: {
            username: "",
            password: "",
        },
        validate: {
            username: (value) =>
                value.length > 0 ? null : t("common.error.requiredField"),
            password: (value) =>
                value.length > 0 ? null : t("common.error.requiredField"),
        },
    });

    return (
        <Center w="100%" h="100%">
            <Stack gap="lg" maw="512px" w="90vw">
                <Paper className="paper-light" p="lg" radius="sm" shadow="sm">
                    <Group gap="sm" justify="space-between">
                        <TbCube size={48} />
                        <Stack gap={4} align="end">
                            <Title order={1} fw={400} mb={0}>
                                {t("lex.appName")}
                            </Title>
                            <Text size="sm" c="dimmed">
                                {t("lex.appSub")}
                            </Text>
                        </Stack>
                    </Group>
                </Paper>
                <Paper className="paper-light" p="lg" radius="sm" shadow="sm">
                    <form
                        onSubmit={form.onSubmit((values) => {
                            AuthenticationService.login({ body: values }).then(
                                (result) => {
                                    if (result.data) {
                                        reload().then(() => nav("/"));
                                    } else {
                                        error(t("views.auth.error"));
                                    }
                                }
                            );
                        })}
                    >
                        <Stack gap="sm">
                            <Group gap="sm" justify="space-between">
                                <TbLogin2 size={32} />
                                <Title order={3} fw={400} m={0}>
                                    {t("action.login")}
                                </Title>
                            </Group>
                            <Divider />
                            <TextInput
                                size="md"
                                leftSection={<TbUser size={20} />}
                                {...form.getInputProps("username")}
                                label={t("views.auth.usernameLabel")}
                            />
                            <PasswordField
                                size="md"
                                leftSection={<TbLock size={20} />}
                                {...form.getInputProps("password")}
                                label={t("views.auth.passwordLabel")}
                            />
                            <Group justify="end">
                                <Button
                                    leftSection={<TbLogin2 size={20} />}
                                    type="submit"
                                >
                                    {t("action.login")}
                                </Button>
                            </Group>
                        </Stack>
                    </form>
                </Paper>
            </Stack>
        </Center>
    );
}
