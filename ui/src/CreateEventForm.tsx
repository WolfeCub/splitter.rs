import { Button, Group, NumberInput, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { FC } from "react";
import { client } from "./main";

export const CreateEventForm: FC = () => {
    const form = useForm({
        mode: 'uncontrolled',
        initialValues: {
            name: '',
            amount: 0.0,
        },
    });

    const submit = async () => {
        const formVals = form.getValues();
        client.upsertEvent({
            event: {
                name: formVals.name,
                amount: formVals.amount,
            },
        });
    };

    return (
        <div>
            <TextInput
                label="Name"
                placeholder="Name"
                key={form.key('name')}
                {...form.getInputProps('name')}
            />
            <NumberInput
                mt="md"
                label="Amount"
                placeholder="Amount"
                key={form.key('amount')}
                {...form.getInputProps('amount')}
            />

            <Group justify="center" mt="xl">
                <Button onClick={submit}>
                    Submit
                </Button>
            </Group>
        </div>
    );
}
