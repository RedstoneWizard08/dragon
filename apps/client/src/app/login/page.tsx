"use client";

import { Metadata } from "next";
import styles from "./page.module.scss";
import { createRef } from "react";
import Link from "next/link";

export const metadata: Metadata = {
    title: "Login - Dragon Panel",
};

export default function Page() {
    const usernameRef = createRef<HTMLInputElement>();
    const passwordRef = createRef<HTMLInputElement>();

    return (
        <main className={styles.main}>
            <form
                action=""
                autoComplete="off"
                className={styles.form}
            >
                <p className={styles.loginTitle}>Login</p>

                <input
                    type="text"
                    name="username"
                    placeholder="Username or Email"
                    ref={usernameRef}
                />

                <input
                    type="password"
                    name="password"
                    placeholder="Enter your password"
                    ref={passwordRef}
                />

                <p className={styles.registerLink}>
                    Don&apos;t have an account?{" "}
                    <Link href="/register">Sign up here</Link>
                </p>
            </form>
        </main>
    );
}
