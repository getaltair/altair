<script lang="ts">
  import { onMount } from 'svelte';
  import Card from '$lib/components/primitives/Card.svelte';
  import Button from '$lib/components/primitives/Button.svelte';
  import { todayQuests } from '$lib/repositories/quest.svelte';
  import { dueToday } from '$lib/repositories/routine.svelte';
  import { getSyncClient } from '$lib/sync';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const greeting = $derived(() => {
    const hour = new Date().getHours();
    if (hour < 12) return 'Good morning';
    if (hour < 17) return 'Good afternoon';
    return 'Good evening';
  });

  const name = $derived(data.user?.display_name ?? 'there');

  // Client-side check-in detection — seeds from server value, overridden by $effect below.
  // svelte-ignore state_referenced_locally
  let hasCheckedIn = $state(data.hasCheckedIn ?? false);

  $effect(() => {
    const client = getSyncClient();
    const today = new Date().toISOString().slice(0, 10); // YYYY-MM-DD
    (async () => {
      for await (const result of client.watch(
        'SELECT id FROM daily_checkins WHERE checkin_date = ? AND deleted_at IS NULL LIMIT 1',
        [today]
      )) {
        hasCheckedIn = (result.rows?._array ?? []).length > 0;
      }
    })();
  });

  const quests = $derived(todayQuests());
  const routines = $derived(dueToday());

  // Keyboard shortcut: "n q" → new quest, "n n" → new note
  let _keyPrefix: string | null = $state(null);
  let _keyTimeout: ReturnType<typeof setTimeout> | null = null;

  $effect(() => {
    function handleKeydown(e: KeyboardEvent) {
      // Ignore when focused in an input/textarea
      const tag = (e.target as HTMLElement)?.tagName;
      if (tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT') return;

      if (_keyPrefix === 'n') {
        if (_keyTimeout) clearTimeout(_keyTimeout);
        _keyPrefix = null;
        if (e.key === 'q') {
          window.location.href = '/guidance/quests/new';
        } else if (e.key === 'n') {
          window.location.href = '/knowledge/notes/new';
        }
        return;
      }

      if (e.key === 'n') {
        _keyPrefix = 'n';
        _keyTimeout = setTimeout(() => {
          _keyPrefix = null;
        }, 1500);
      }
    }

    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });
</script>

<div class="today">
  <header class="today__header">
    <h1 class="greeting">{greeting()}, {name}.</h1>
  </header>

  {#if !hasCheckedIn}
    <section class="today__section">
      <Card>
        <div class="checkin-card">
          <p class="checkin-card__label">Daily Check-In</p>
          <p class="checkin-card__text">Take a moment to reflect on today.</p>
          <Button onclick={() => (window.location.href = '/guidance/checkins/new')}>
            Start Check-In
          </Button>
        </div>
      </Card>
    </section>
  {/if}

  <section class="today__section">
    <h2 class="section-title">Due Routines</h2>
    {#if routines.length === 0}
      <p class="empty-state">No routines due today.</p>
    {:else}
      <ul class="item-list">
        {#each routines as routine (routine.id)}
          <li>
            <a href="/guidance/routines/{routine.id}" class="item-link">
              <Card>
                <div class="item-row">
                  <span class="item-title">{routine.title}</span>
                  <span class="item-meta">{routine.frequency_type}</span>
                </div>
              </Card>
            </a>
          </li>
        {/each}
      </ul>
    {/if}
  </section>

  <section class="today__section">
    <h2 class="section-title">Today's Quests</h2>
    {#if quests.length === 0}
      <p class="empty-state">No quests for today.</p>
    {:else}
      <ul class="item-list">
        {#each quests as quest (quest.id)}
          <li>
            <a href="/guidance/quests/{quest.id}" class="item-link">
              <Card>
                <div class="item-row">
                  <span class="item-title">{quest.title}</span>
                  <span class="item-meta">{quest.status}</span>
                </div>
              </Card>
            </a>
          </li>
        {/each}
      </ul>
    {/if}
  </section>

  <div class="quick-actions">
    <Button onclick={() => (window.location.href = '/guidance/quests/new')}>
      New Quest <kbd>n q</kbd>
    </Button>
    <Button variant="secondary" onclick={() => (window.location.href = '/knowledge/notes/new')}>
      New Note <kbd>n n</kbd>
    </Button>
  </div>
</div>

<style>
  .today {
    max-width: 48rem;
    margin: 0 auto;
    padding: 2rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .greeting {
    font-family: var(--font-display);
    font-size: clamp(1.75rem, 4vw, 2.5rem);
    font-weight: 700;
    color: var(--on-surface);
    line-height: 1.2;
  }

  .section-title {
    font-family: var(--font-display);
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--on-surface);
    margin-bottom: 0.75rem;
  }

  .empty-state {
    color: var(--on-surface-variant);
    font-size: 0.9375rem;
  }

  .item-list {
    list-style: none;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .item-link {
    text-decoration: none;
    display: block;
  }

  .item-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .item-title {
    font-weight: 600;
    color: var(--on-surface);
  }

  .item-meta {
    font-size: 0.8125rem;
    color: var(--on-surface-variant);
    text-transform: capitalize;
  }

  .checkin-card {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .checkin-card__label {
    font-family: var(--font-display);
    font-weight: 700;
    color: var(--on-surface);
  }

  .checkin-card__text {
    color: var(--on-surface-variant);
    font-size: 0.9375rem;
  }

  .quick-actions {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  kbd {
    font-family: var(--font-body);
    font-size: 0.75rem;
    opacity: 0.6;
    margin-left: 0.25rem;
  }
</style>
