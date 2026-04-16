<script lang="ts">
  import Button from '$lib/components/primitives/Button.svelte';
  import { questById } from '$lib/repositories/quest.svelte';
  import { getSyncClient } from '$lib/sync';

  const DURATION_MS = 25 * 60 * 1000; // 25 minutes
  const SOFT_SLATE_HAZE = '#cfddde';

  let { data }: { data: { id: string } } = $props();

  const quest = $derived(questById(data.id));

  type TimerState = 'idle' | 'running' | 'paused' | 'done';

  let timerState = $state<TimerState>('idle');
  let startTime = $state<number | null>(null);
  let pausedElapsed = $state(0); // ms elapsed before current run
  let elapsed = $state(0);       // total ms elapsed (updated on tick)

  const remaining = $derived(Math.max(0, DURATION_MS - elapsed));
  const minutes = $derived(Math.floor(remaining / 60000));
  const seconds = $derived(Math.floor((remaining % 60000) / 1000));
  const progress = $derived(elapsed / DURATION_MS);

  // SVG ring parameters
  const RADIUS = 90;
  const CIRCUMFERENCE = 2 * Math.PI * RADIUS;
  const strokeDashoffset = $derived(CIRCUMFERENCE * (1 - Math.min(progress, 1)));

  $effect(() => {
    if (timerState !== 'running') return;

    const intervalId = setInterval(() => {
      if (startTime === null) return;
      elapsed = pausedElapsed + (Date.now() - startTime);

      if (elapsed >= DURATION_MS) {
        elapsed = DURATION_MS;
        timerState = 'done';
        document.body.style.backgroundColor = '';
        onSessionComplete();
      }
    }, 1000);

    return () => clearInterval(intervalId);
  });

  $effect(() => {
    if (timerState === 'running') {
      document.body.style.backgroundColor = SOFT_SLATE_HAZE;
    } else {
      document.body.style.backgroundColor = '';
    }
    return () => {
      document.body.style.backgroundColor = '';
    };
  });

  function start() {
    startTime = Date.now();
    timerState = 'running';
  }

  function pause() {
    if (startTime !== null) {
      pausedElapsed += Date.now() - startTime;
    }
    startTime = null;
    timerState = 'paused';
  }

  function stop() {
    pausedElapsed = 0;
    startTime = null;
    elapsed = 0;
    timerState = 'idle';
  }

  async function onSessionComplete() {
    if (!quest) return;
    const client = getSyncClient();
    const sessionId = crypto.randomUUID();
    const now = new Date().toISOString();
    const sessionStarted = new Date(Date.now() - DURATION_MS).toISOString();
    await client.execute(
      `INSERT INTO focus_sessions (id, quest_id, started_at, ended_at, duration_minutes, user_id, created_at, updated_at)
       VALUES (?, ?, ?, ?, 25, (SELECT user_id FROM quests WHERE id = ? LIMIT 1), ?, ?)`,
      [sessionId, quest.id, sessionStarted, now, quest.id, now, now]
    );
  }

  const pad = (n: number) => String(n).padStart(2, '0');
</script>

<div class="page">
  <nav class="breadcrumb">
    <a href="/guidance/quests/{data.id}" class="breadcrumb__link">
      {quest?.title ?? 'Quest'}
    </a>
    <span class="breadcrumb__sep">/</span>
    <span class="breadcrumb__current">Focus Session</span>
  </nav>

  <div class="timer-area">
    <div class="ring-wrapper" aria-label="Focus timer progress">
      <svg class="ring" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
        <!-- Background track -->
        <circle
          cx="100"
          cy="100"
          r={RADIUS}
          fill="none"
          stroke="var(--surface-highest)"
          stroke-width="10"
        />
        <!-- Progress arc with gradient -->
        <defs>
          <linearGradient id="ring-gradient" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="var(--primary)" />
            <stop offset="100%" stop-color="var(--primary-container)" />
          </linearGradient>
        </defs>
        <circle
          cx="100"
          cy="100"
          r={RADIUS}
          fill="none"
          stroke="url(#ring-gradient)"
          stroke-width="10"
          stroke-linecap="round"
          stroke-dasharray={CIRCUMFERENCE}
          stroke-dashoffset={strokeDashoffset}
          transform="rotate(-90 100 100)"
          style="transition: stroke-dashoffset var(--motion-standard)"
        />
      </svg>

      <div class="timer-display">
        {#if timerState === 'done'}
          <span class="timer-done">Done!</span>
        {:else}
          <span class="timer-time">{pad(minutes)}:{pad(seconds)}</span>
        {/if}
        <span class="timer-label">
          {timerState === 'idle'
            ? '25 min'
            : timerState === 'running'
              ? 'Focus'
              : timerState === 'paused'
                ? 'Paused'
                : 'Complete'}
        </span>
      </div>
    </div>

    <div class="controls">
      {#if timerState === 'idle'}
        <Button onclick={start}>Start</Button>
      {:else if timerState === 'running'}
        <Button variant="secondary" onclick={pause}>Pause</Button>
        <Button variant="ghost" onclick={stop}>Stop</Button>
      {:else if timerState === 'paused'}
        <Button onclick={start}>Resume</Button>
        <Button variant="ghost" onclick={stop}>Stop</Button>
      {:else}
        <Button onclick={stop}>Start New</Button>
        <a href="/guidance/quests/{data.id}" class="back-link">
          <Button variant="ghost">Back to Quest</Button>
        </a>
      {/if}
    </div>
  </div>
</div>

<style>
  .page {
    max-width: 32rem;
    margin: 0 auto;
    padding: 2rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .breadcrumb {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.875rem;
  }

  .breadcrumb__link {
    color: var(--primary);
    text-decoration: none;
  }

  .breadcrumb__link:hover {
    text-decoration: underline;
  }

  .breadcrumb__sep {
    color: var(--outline-variant);
  }

  .breadcrumb__current {
    color: var(--on-surface-variant);
  }

  .timer-area {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2rem;
  }

  .ring-wrapper {
    position: relative;
    width: 220px;
    height: 220px;
  }

  .ring {
    width: 220px;
    height: 220px;
  }

  .timer-display {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.25rem;
  }

  .timer-time {
    font-family: var(--font-display);
    font-size: 2.5rem;
    font-weight: 700;
    color: var(--on-surface);
    letter-spacing: -0.02em;
    font-variant-numeric: tabular-nums;
  }

  .timer-done {
    font-family: var(--font-display);
    font-size: 2rem;
    font-weight: 700;
    color: var(--primary);
  }

  .timer-label {
    font-size: 0.875rem;
    color: var(--on-surface-variant);
    font-weight: 500;
  }

  .controls {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }

  .back-link {
    text-decoration: none;
  }
</style>
