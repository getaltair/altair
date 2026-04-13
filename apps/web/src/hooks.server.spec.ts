import { describe, it, expect, vi } from "vitest";
import { handle } from "./hooks.server";

// Helpers to build a minimal SvelteKit event stub
function makeEvent(cookieValue: string | undefined) {
  return {
    cookies: {
      get: (name: string) => (name === "access_token" ? cookieValue : undefined),
    },
    locals: {} as App.Locals,
  };
}

function makeResolve() {
  return vi.fn(async (_event: unknown) => new Response("ok"));
}

// Build a real base64url-encoded JWT with the given payload
function buildJwt(payload: Record<string, unknown>): string {
  const header = btoa(JSON.stringify({ alg: "HS256", typ: "JWT" }))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");
  const body = btoa(JSON.stringify(payload))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");
  return `${header}.${body}.fakesignature`;
}

describe("hooks.server handle — JWT decode", () => {
  it("sets locals.user to null when access_token cookie is absent", async () => {
    const event = makeEvent(undefined);
    await handle({ event: event as any, resolve: makeResolve() });
    expect(event.locals.user).toBeNull();
  });

  it("sets locals.user to null for a truncated/malformed cookie (no dots)", async () => {
    // No '.' separator → split gives a single element; payloadB64 is undefined
    const event = makeEvent("notavalidjwt");
    await handle({ event: event as any, resolve: makeResolve() });
    expect(event.locals.user).toBeNull();
  });

  it("sets locals.user to null for a JWT with non-JSON payload", async () => {
    // header.payload.sig where payload base64-decodes to non-JSON
    const nonJsonB64 = btoa("this is not json");
    const event = makeEvent(`eyJhbGciOiJIUzI1NiJ9.${nonJsonB64}.fakesig`);
    await handle({ event: event as any, resolve: makeResolve() });
    expect(event.locals.user).toBeNull();
  });

  it("sets locals.user to null when token is missing exp claim", async () => {
    const jwt = buildJwt({ sub: "user-123", email: "test@example.com" });
    const event = makeEvent(jwt);
    await handle({ event: event as any, resolve: makeResolve() });
    expect(event.locals.user).toBeNull();
  });

  it("sets locals.user to null when token is missing sub claim", async () => {
    const futureExp = Math.floor(Date.now() / 1000) + 3600;
    const jwt = buildJwt({ email: "test@example.com", exp: futureExp });
    const event = makeEvent(jwt);
    await handle({ event: event as any, resolve: makeResolve() });
    expect(event.locals.user).toBeNull();
  });

  it("sets locals.user to null when token is expired", async () => {
    const pastExp = Math.floor(Date.now() / 1000) - 1;
    const jwt = buildJwt({ sub: "user-123", email: "test@example.com", exp: pastExp });
    const event = makeEvent(jwt);
    await handle({ event: event as any, resolve: makeResolve() });
    expect(event.locals.user).toBeNull();
  });

  it("populates locals.user with id and email from a valid JWT", async () => {
    const futureExp = Math.floor(Date.now() / 1000) + 3600;
    const jwt = buildJwt({
      sub: "user-abc-123",
      email: "hello@example.com",
      exp: futureExp,
    });
    const event = makeEvent(jwt);
    await handle({ event: event as any, resolve: makeResolve() });
    expect(event.locals.user).toEqual({ id: "user-abc-123", email: "hello@example.com" });
  });

  it("populates locals.user with empty email when email claim is absent", async () => {
    const futureExp = Math.floor(Date.now() / 1000) + 3600;
    const jwt = buildJwt({ sub: "user-no-email", exp: futureExp });
    const event = makeEvent(jwt);
    await handle({ event: event as any, resolve: makeResolve() });
    expect(event.locals.user).toEqual({ id: "user-no-email", email: "" });
  });

  it("calls resolve with the event after processing", async () => {
    const futureExp = Math.floor(Date.now() / 1000) + 3600;
    const jwt = buildJwt({ sub: "user-xyz", email: "a@b.com", exp: futureExp });
    const event = makeEvent(jwt);
    const resolve = makeResolve();
    await handle({ event: event as any, resolve });
    expect(resolve).toHaveBeenCalledOnce();
    expect(resolve).toHaveBeenCalledWith(event);
  });
});
