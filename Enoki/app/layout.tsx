import "./globals.css";
import { Inter } from "next/font/google";
import { NextFont } from "next/dist/compiled/@next/font";
import { Metadata } from "next";
import React from "react";

const inter: NextFont = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Enoki",
  description: "A Tauri Based Front End For FRC Libraries",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>{children}</body>
    </html>
  );
}
