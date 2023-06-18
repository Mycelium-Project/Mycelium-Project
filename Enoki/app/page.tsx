"use client";

import Image from "next/image";
import { JSX } from "react";
import {
  DoesNetworkTableClientExist,
  NetworkTableClientId,
  StartNetworkTableClient,
  TableEntry,
} from "@/utilities/NetworkTableV4";
import { invoke } from "@tauri-apps/api/tauri";
import { window } from "@tauri-apps/api";
import { TauriEvent } from "@tauri-apps/api/event";
import NetworkTable from "@/app/components/network_table";
import { TraceWarn } from "@/utilities/Tracing";
import { LargeButton } from "@/app/components/buttons";

window
  .getCurrent()
  .listen(TauriEvent.WINDOW_CLOSE_REQUESTED, (): boolean => {
    invoke("close").then();
    return true;
  })
  .then();

export default function Home(): JSX.Element {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="z-10 w-full max-w-5xl items-center justify-between font-mono text-sm lg:flex">
        <p className="fixed left-0 top-0 flex w-full justify-center border-b border-gray-300 bg-gradient-to-b from-zinc-200 pb-6 pt-8 backdrop-blur-2xl dark:border-neutral-800 dark:bg-zinc-800/30 dark:from-inherit lg:static lg:w-auto  lg:rounded-xl lg:border lg:bg-gray-200 lg:p-4 lg:dark:bg-zinc-800/30">
          Get started by opening&nbsp;
          <code className="font-mono font-bold">docs/developing.md</code>
        </p>
      </div>

      <div className="relative flex place-items-center before:absolute before:h-[300px] before:w-[480px] before:-translate-x-1/2 before:rounded-full before:bg-gradient-radial before:from-white before:to-transparent before:blur-2xl before:content-[''] after:absolute after:-z-20 after:h-[180px] after:w-[240px] after:translate-x-1/3 after:bg-gradient-conic after:from-sky-200 after:via-blue-200 after:blur-2xl after:content-[''] before:dark:bg-gradient-to-br before:dark:from-transparent before:dark:to-blue-700 before:dark:opacity-10 after:dark:from-sky-900 after:dark:via-[#0141ff] after:dark:opacity-40 before:lg:h-[360px]">
        <Image
          className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
          src="/next.svg"
          alt="Next.js Logo"
          width={180}
          height={37}
          priority
        />
      </div>

      <NetworkTable />

      <div className="mb-32 grid text-center lg:mb-0 lg:grid-cols-3 lg:text-left">
        <LargeButton
          title="Connect"
          subtext="Click here to connect to the network tables server on localhost:5810"
          action={StartNTClient}
        />

        <LargeButton
          title="Disconnect"
          subtext="Click here to disconnect from the network tables server on localhost:5810"
          action={StopNTClient}
        />

        <LargeButton
          title="Subscribe"
          subtext="Click here to subscribe to a value on the network tables server on localhost:5810"
          action={SubscribeExample}
        />

        <LargeButton
          action={PublishExample}
          title={"Publish"}
          subtext={
            "Click here to publish a value on the network tables server on localhost:5810"
          }
        />

        <LargeButton
          action={DoesClientExist}
          title={"Is Connected?"}
          subtext={
            "Click here to check if a handler exists for the network tables server on localhost:5810"
          }
        />

        <LargeButton
          action={PollSubscriptions}
          title={"Get Subbed"}
          subtext={
            "Click here to poll all subbed data on the network tables server on localhost:5810"
          }
        />
      </div>
    </main>
  );
}

//create a test table variable
let testTable: NetworkTableClientId;

async function StartNTClient(): Promise<void> {
  console.log("Starting NetworkTables");
  TraceWarn("TEST");
  testTable = await StartNetworkTableClient([127, 0, 0, 1], 5810, "Enoki-test");
}

function StopNTClient(): void {
  console.log("Stopping NetworkTables");
  if (testTable) {
    testTable.stopNetworkTableClient();
  } else {
    TraceWarn("No client to stop");
  }
}

function DoesClientExist(): void {
  DoesNetworkTableClientExist(testTable).then((result: boolean) =>
    console.log(result)
  );
}

function SubscribeExample(): void {
  console.log("Subscribing to NetworkTables");
  testTable.subscribe("", 0.05, true, true);
}

function PublishExample(): void {
  console.log("Publishing to NetworkTables");
  testTable.setEntry("/test", 1);
}

async function PollSubscriptions(): Promise<void> {
  console.log("Polling Subscriptions");
  let entries: TableEntry[] = await testTable.getEntries();
  console.log(entries);
}
