import type { Metadata } from "next";
import "./globals.scss";

export const metadata: Metadata = {
    title: "Dragon Panel",
    description: "A simple, blazingly fast, effortlessly secure, and extensible game server control panel.",
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en">
            <body>{children}</body>
        </html>
    );
}
