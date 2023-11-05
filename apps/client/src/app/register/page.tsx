"use client";

import { Metadata } from "next";
import styles from "./page.module.scss";
import { createRef } from "react";
import Link from "next/link";

export const metadata: Metadata = {
    title: "Sign Up - Dragon Panel",
};

export default function Page() {
    const usernameRef = createRef<HTMLInputElement>();
    const emailRef = createRef<HTMLInputElement>();
    const passwordRef = createRef<HTMLInputElement>();
    const passwordConfirmRef = createRef<HTMLInputElement>();

    return (
        <main className={styles.main}>
            <form
                action=""
                autoComplete="off"
                className={styles.form}
            >
                <p className={styles.registerTitle}>Sign Up</p>

                <input
                    type="text"
                    name="username"
                    placeholder="Username"
                    ref={usernameRef}
                />

                <input
                    type="email"
                    name="email"
                    placeholder="Email"
                    ref={emailRef}
                />

                <input
                    type="password"
                    name="password"
                    placeholder="Enter your password"
                    ref={passwordRef}
                />

                <input
                    type="password"
                    name="confirmPassword"
                    placeholder="Confirm your password"
                    ref={passwordConfirmRef}
                />

                <p className={styles.loginLink}>
                    Already have an account?{" "}
                    <Link href="/login">Sign in here</Link>
                </p>
            </form>
        </main>
    );
}
