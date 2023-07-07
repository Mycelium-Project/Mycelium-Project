import { Card, Tooltip, Typography } from "@material-tailwind/react";
import { JSX } from "react";
import { DisplayTableEntry } from "@/utilities/NetworkTableV4";
import Image from "next/image";

const TABLE_HEAD: string[] = ["Key", "Type", "Value", "Age (s)", ""];

// TODO: Swap this from demo data to live data from rust handler
const TABLE_ROWS: DisplayTableEntry[] = [
  {
    key: "NT | SmartDashboard | Battery Voltage",
    type: "Double",
    value: "11.963",
    last_updated: 0,
    client_id: "Localhost:5800",
  },
  {
    key: "NT | SmartDashboard | Shots Fired",
    type: "Int",
    value: "32",
    last_updated: 2.3,
    client_id: "Localhost:5800",
  },
  {
    key: "NT | Shuffleboard | Pose",
    type: "Double[]",
    value: "[2.334, 4.674, 120.566]",
    last_updated: 0,
    client_id: "Localhost:5800",
  },
  {
    key: "NT | FMS Info | FMS Mode",
    type: "int",
    value: "42",
    last_updated: 64,
    client_id: "Localhost:5800",
  },
  {
    key: "NT | Mycelium | Logger Running",
    type: "Boolean",
    value: "true",
    last_updated: 64,
    client_id: "Localhost:5800",
  },
];

// TODO: Fix TABLE_HEAD to change size as columns vanish looks funky sometimes on narrow screens
export default function NetworkTable(): JSX.Element {
  return (
    <Card className="h-full max-w-fit">
      <table className="w-full min-w-max table-auto text-left">
        <thead>
          <tr>
            {TABLE_HEAD.map((head: string) => (
              <th
                key={head}
                className="border-b border-blue-gray-100 bg-blue-gray-50 p-4"
              >
                <Typography
                  variant="small"
                  color="blue-gray"
                  className="font-normal leading-none opacity-70 hidden md:table-cell"
                >
                  {head}
                </Typography>
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {TABLE_ROWS.map(
            ({ key, type, value, last_updated }, index: number) => (
              <tr key={key} className="even:bg-blue-gray-50/50">
                <td className="p-4">
                  <Typography
                    variant="small"
                    color="blue-gray"
                    className="font-normal overflow-ellipsis"
                  >
                    {key}
                  </Typography>
                </td>
                <td className="p-4 hidden sm:table-cell">
                  <Typography
                    variant="small"
                    color="blue-gray"
                    className="font-normal"
                  >
                    {type}
                  </Typography>
                </td>
                <td className="p-4">
                  <Typography
                    variant="small"
                    color="blue-gray"
                    className="font-normal"
                  >
                    {value}
                  </Typography>
                </td>
                <td className="p-4 hidden md:table-cell">
                  <Typography
                    variant="small"
                    color="blue-gray"
                    className="font-normal"
                  >
                    {last_updated}
                  </Typography>
                </td>
                <td className="p-4 hidden sm:table-cell">
                  <Tooltip content="Copy to clipboard" placement="bottom">
                    <Image
                      className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
                      src="../copy.svg"
                      alt="Copy"
                      width={25}
                      height={25}
                      priority
                    />
                  </Tooltip>
                </td>
              </tr>
            ),
          )}
        </tbody>
      </table>
    </Card>
  );
}
