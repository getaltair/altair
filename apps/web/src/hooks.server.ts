import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
  const accessToken = event.cookies.get("access_token");

  if (accessToken) {
    try {
      // Decode JWT payload (no verification — verification happens server-side on API calls)
      const [, payloadB64] = accessToken.split(".");
      const payload = JSON.parse(
        atob(payloadB64.replace(/-/g, "+").replace(/_/g, "/")),
      );
      if (
        payload.sub &&
        payload.exp &&
        payload.exp > Math.floor(Date.now() / 1000)
      ) {
        event.locals.user = {
          id: payload.sub,
          email: payload.email ?? "",
        };
      } else {
        event.locals.user = null;
      }
    } catch (e) {
      // P4-005: log decode failures so cookie corruption or proxy truncation is
      // visible in server logs rather than silently degrading auth for all users.
      console.error("[hooks.server] Failed to decode access_token cookie:", e);
      event.locals.user = null;
    }
  } else {
    event.locals.user = null;
  }

  return resolve(event);
};
