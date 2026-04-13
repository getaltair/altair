// Source of truth: packages/contracts/sync-streams.json
// Note: these stream names are provisional — Step 4 (Sync Engine) may revise them.
export const SyncStream = {
  UserData:  'user_data',
  Household: 'household',
  Guidance:  'guidance',
  Knowledge: 'knowledge',
  Tracking:  'tracking',
} as const;

export type SyncStreamValue = typeof SyncStream[keyof typeof SyncStream];
