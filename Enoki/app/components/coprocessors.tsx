import { Card } from "@material-tailwind/react";
import { JSX } from "react";
import Link from "next/link";

export function CoprocessorPurposeCard(props: any): JSX.Element {
  return (
    <Link href={"/coprocessor/" + props.name}>
      <Card className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30">
        <h2 className={`mb-3 text-2xl font-semibold`}>
          {props.name + " "}
          <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
            -&gt;
          </span>
        </h2>
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
          {"Address: " + props.ip + ":" + props.port}
        </p>
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
          {"Purpose: " + props.purpose}
        </p>
        <br />
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>{props.note}</p>
      </Card>
    </Link>
  );
}

export function CoprocessorUsageCard(props: any): JSX.Element {
  return (
    <Link href={"/coprocessor/" + props.name}>
      <Card className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30">
        <h2 className={`mb-3 text-2xl font-semibold`}>
          {props.name + " "}
          <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
            -&gt;
          </span>
        </h2>
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
          {"CPU: " + props.cpu + "%"}
        </p>
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
          {"RAM: " + props.ram + " MB"}
        </p>
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
          {"Network: " + props.network + " MB"}
        </p>
        <br />
        <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>{props.note}</p>
      </Card>
    </Link>
  );
}

export function CoprocessorFullCard(props: any): JSX.Element {
  return (
    <Link href={"/coprocessor/" + props.name}>
      <Card className="group rounded-lg border border-transparent px-5 py-4 transition-colors hover:border-gray-300 hover:bg-gray-100 hover:dark:border-neutral-700 hover:dark:bg-neutral-800/30">
        <h2 className={`mb-3 text-2xl font-semibold`}>
          {props.name + " "}
          <span className="inline-block transition-transform group-hover:translate-x-1 motion-reduce:transform-none">
            -&gt;
          </span>
        </h2>
        <div className="grid text-center lg:mb-0 lg:grid-cols-2 lg:text-left">
          <div className="pr-2">
            <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
              {"Address: " + props.ip + ":" + props.port}
            </p>
            <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
              {"Purpose: " + props.purpose}
            </p>
            <br />
          </div>
          <div className="pl-2">
            <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
              {"CPU: " + props.cpu + "%"}
            </p>
            <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
              {"RAM: " + props.ram + " MB"}
            </p>
            <p className={`m-0 max-w-[30ch] text-sm opacity-50`}>
              {"Network: " + props.network + " MB"}
            </p>
            <br />
          </div>
        </div>
        <p className={`m-0 max-w-[65ch] text-sm opacity-50`}>{props.note}</p>
      </Card>
    </Link>
  );
}
