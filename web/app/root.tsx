import { Box, ScrollArea } from "@mantine/core";
import { StylesPlaceholder } from "@mantine/remix";
import type { MetaFunction } from "@remix-run/cloudflare";
import {
  Links,
  LiveReload,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import Providers from "./context/Providers";

export const meta: MetaFunction = () => ({
  charset: "utf-8",
  title: "Politics & War",
  viewport: "width=device-width,initial-scale=1",
});

function InnerApp() {
  return (
    <>
      <ScrollArea.Autosize maxHeight={`calc(100vh - 0px)`}>
        <Box p="md" sx={{ flexGrow: 1 }}>
          <Outlet />
        </Box>
      </ScrollArea.Autosize>
    </>
  );
}

export default function App() {
  return (
    <Providers>
      <html lang="en">
        <head>
          <Meta />
          <Links />
          <StylesPlaceholder />
        </head>
        <body>
          <InnerApp />
          <ScrollRestoration />
          <Scripts />
          <LiveReload />
        </body>
      </html>
    </Providers>
  );
}
