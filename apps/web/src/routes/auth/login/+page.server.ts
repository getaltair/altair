import { fail, redirect } from "@sveltejs/kit";
import { dev } from "$app/environment";
import type { Actions } from "./$types";
import { PUBLIC_API_BASE_URL } from "$env/static/public";

export const actions: Actions = {
  default: async ({ request, cookies }) => {
    const data = await request.formData();
    const email = data.get("email") as string;
    const password = data.get("password") as string;

    if (!email || !password) {
      return fail(400, { error: "Email and password are required" });
    }

    let response: Response;
    try {
      response = await fetch(`${PUBLIC_API_BASE_URL}/api/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ email, password }),
      });
    } catch {
      return fail(503, { error: "Server unavailable. Please try again." });
    }

    if (response.ok) {
      // P4-011: guard against malformed server response before using the body.
      let body: { access_token: string; refresh_token: string };
      try {
        body = await response.json();
      } catch {
        return fail(500, { error: "Server returned an unexpected response." });
      }
      if (!body.access_token || !body.refresh_token) {
        return fail(500, { error: "Server returned incomplete token data." });
      }

      // P4-006: Secure attribute omitted in dev (HTTP), enabled in production (HTTPS).
      cookies.set("access_token", body.access_token, {
        httpOnly: true,
        sameSite: "lax",
        path: "/",
        maxAge: 900,
        secure: !dev,
      });
      cookies.set("refresh_token", body.refresh_token, {
        httpOnly: true,
        sameSite: "lax",
        path: "/api/auth/",
        maxAge: 604800,
        secure: !dev,
      });
      redirect(303, "/");
    }

    if (response.status === 401) {
      return fail(401, { error: "Invalid email or password" });
    }
    if (response.status === 403) {
      return fail(403, { error: "Account pending admin approval" });
    }

    return fail(response.status, { error: "Login failed. Please try again." });
  },
};
