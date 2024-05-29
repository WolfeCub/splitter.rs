import { FC, useEffect, useState } from "react"
import { client } from "./main";
import { Event } from "./proto/splitter";
import { useDisclosure } from '@mantine/hooks';
import { Modal, Button } from '@mantine/core';
import { CreateEventForm } from "./CreateEventForm";

export const PageContent: FC = () => {
    const [opened, { open, close }] = useDisclosure(false);
    const [events, setEvents] = useState<Event[]>([]);

    useEffect(() => {
        client.getAllEvents({})
            .then((r) => setEvents(r.response.events))
            .catch((e) => console.error(e));
    }, []);


    return (
        <>
            <Modal opened={opened} onClose={close} title="Event">
                <CreateEventForm />
            </Modal>

            <Button onClick={open}>Add event</Button>

            <ul>
                {events.map((e) =>
                    <li key={e.id}>
                        {e.name} - {e.amount}
                    </li>
                )}
            </ul>
        </>
    );
}

