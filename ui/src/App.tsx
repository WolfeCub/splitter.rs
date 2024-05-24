import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport"
import { useEffect } from "react"
import { SplitterClient } from "./proto/splitter.client";

export const App = () => {
    useEffect(() => {
        let transport = new GrpcWebFetchTransport({
            baseUrl: "http://localhost:3000"
        });

        let client = new SplitterClient(transport);

        const blah = async () => {
            const response = await client.createAccount({});
            console.log(response);
        }
        blah();

    }, []);

    return (
        <>
            <div>Hello</div>
        </>
    )
}

