import { FC, useEffect, useState } from "react"
import { client } from "./main";
import { Event } from "./proto/splitter";
import { useDisclosure } from '@mantine/hooks';
import { Modal, Button, Table } from '@mantine/core';
import { CreateEventForm } from "./CreateEventForm";

export const PageContent: FC = () => {
    const [opened, { open, close }] = useDisclosure(false);
    const [events, setEvents] = useState<Event[]>([]);

    const fetchEvents = () => {
        client.getAllEvents({})
            .then((r) => setEvents(r.response.events))
            .catch((e) => console.error(e));
    };

    const closeModal = () => {
        fetchEvents();
        close();
    };

    useEffect(fetchEvents, []);

    const rows = events.map((event) => (
        <Table.Tr key={event.id}>
        <Table.Td>{event.name}</Table.Td>
        <Table.Td>{event.amount}</Table.Td>
        </Table.Tr>
    ));

    return (
        <>
            <Modal opened={opened} onClose={close} title="Event">
                <CreateEventForm close={closeModal} />
            </Modal>

            <Button onClick={open}>Add event</Button>

            <Table>
                <Table.Thead>
                    <Table.Tr>
                        <Table.Th>Name</Table.Th>
                        <Table.Th>Amount</Table.Th>
                    </Table.Tr>
                </Table.Thead>
                <Table.Tbody>{rows}</Table.Tbody>
            </Table>
        </>
    );
}

