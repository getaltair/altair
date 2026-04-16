# ADR-023: Android minSdk = 31 (Deviation from Platform Spec)

| Field | Value |
|---|---|
| **Status** | Accepted |
| **Date** | 2026-04-16 |
| **Feature** | 009-AndroidClient |

## Context

`docs/specs/09-PLAT-001-android.md` states "Android 8+ (API 26+)" as the target minimum SDK. However, the existing `apps/android/app/build.gradle.kts` already declares `minSdk = 31` (Android 12).

Dropping to API 26 would require:
- Adding compatibility shims or conditional branches for APIs introduced between 26 and 31 (e.g., `SplashScreen` API, improved Bluetooth permissions, exact alarm permissions, notification permission runtime request at 33+)
- Verified testing on API 26–30 emulator images
- Additional Compose compatibility surface (Compose's minimum is API 21 but several Material 3 features are smoother at 31+)
- EncryptedSharedPreferences behavior differences (though it functions from API 23+, edge cases exist at lower APIs)

Platform spec stated API 26+ as a general guidance level; it was written before the scaffold was created with `minSdk = 31`.

## Decision

**Accept `minSdk = 31` as the effective minimum, superseding the platform spec's API 26 guidance.**

API 31 (Android 12) represents approximately 95–97% of active Android devices as of 2026. The marginal addressable market from supporting API 26–30 does not justify the development and maintenance cost of multi-API compatibility branches throughout the codebase.

`docs/specs/09-PLAT-001-android.md` is a guidance document; this ADR is the authoritative record of the actual minimum. FA-015 in `Context/Features/009-AndroidClient/Spec.md` was updated to reference `minSdk 31`.

## Consequences

- No `Build.VERSION.SDK_INT` conditional branches needed for APIs introduced between 26 and 31.
- EncryptedSharedPreferences (API 23+) works without edge-case handling at API 31+.
- WorkManager, Compose BOM, and all planned dependencies are tested and stable at API 31+.
- Devices running Android 8–11 (API 26–30) are excluded from the Altair Android client.
- If future market analysis changes the distribution picture, raising this to API 33+ (requiring the `POST_NOTIFICATIONS` runtime permission for Focus timer — already planned in the Spec's Should Have section) should be revisited.
