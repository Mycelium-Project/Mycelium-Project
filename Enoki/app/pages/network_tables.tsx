import { Card, Typography } from "@material-tailwind/react";
import {JSX} from "react";

const TABLE_HEAD: string[] = ["Key", "Type", "Value", "Age (s)", ""];

const TABLE_ROWS = [
    {
        key: "NT | SmartDashboard | Battery Voltage",
        type: "Double",
        value: "11.963",
        last_updated: 0
    },
    {
        key: "NT | SmartDashboard | Shots Fired",
        type: "Int",
        value: "32",
        last_updated: 2.3
    },
    {
        key: "NT | Shuffleboard | Pose",
        type: "Double[]",
        value: "[2.334, 4.674, 120.566]",
        last_updated: 0
    },
    {
        key: "NT | FMS Info | FMS Mode",
        type: "int",
        value: "42",
        last_updated: 64
    },
    {
        key: "NT | Mycelium | Logger Running",
        type: "Boolean",
        value: "true",
        last_updated: 64
    },
];

export default function Test(): JSX.Element {
    return (
        <Card className="h-full w-full">
            <table className="w-full min-w-max table-auto text-left">
                <thead>
                <tr>
                    {TABLE_HEAD.map((head: string) => (
                        <th key={head} className="border-b border-blue-gray-100 bg-blue-gray-50 p-4">
                            <Typography
                                variant="small"
                                color="blue-gray"
                                className="font-normal leading-none opacity-70"
                            >
                                {head}
                            </Typography>
                        </th>
                    ))}
                </tr>
                </thead>
                <tbody>
                {TABLE_ROWS.map(({ key, type, value, last_updated }, index: number) => (
                    <tr key={key} className="even:bg-blue-gray-50/50">
                        <td className="p-4">
                            <Typography variant="small" color="blue-gray" className="font-normal">
                                {key}
                            </Typography>
                        </td>
                        <td className="p-4">
                            <Typography variant="small" color="blue-gray" className="font-normal">
                                {type}
                            </Typography>
                        </td>
                        <td className="p-4">
                            <Typography variant="small" color="blue-gray" className="font-normal">
                                {value}
                            </Typography>
                        </td>
                        <td className="p-4">
                            <Typography variant="small" color="blue-gray" className="font-normal">
                                {last_updated}
                            </Typography>
                        </td>
                        <td className="p-4">
                            <Typography as="a" href="#" variant="small" color="blue" className="font-medium">
                                Copy
                            </Typography>
                        </td>
                    </tr>
                ))}
                </tbody>
            </table>
        </Card>
    );
}