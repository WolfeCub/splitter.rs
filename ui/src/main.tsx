import React from 'react'
import ReactDOM from 'react-dom/client'

import './index.css'
import '@mantine/core/styles.css';
import { MantineProvider } from '@mantine/core';

import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport"
import { SplitterClient } from "./proto/splitter.client";

import { App } from './App.tsx'

let transport = new GrpcWebFetchTransport({
    baseUrl: "http://localhost:3000"
});

export let client = new SplitterClient(transport);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <MantineProvider>
        <App />
    </MantineProvider>
  </React.StrictMode>,
)
