<script lang="ts">
  interface Props {
    value?: string;
    label?: string;
    type?: string;
    error?: string;
    helperText?: string;
    id?: string;
  }

  let {
    value = $bindable(''),
    label,
    type = 'text',
    error,
    helperText,
    id,
  }: Props = $props();

  const inputId = $derived(id ?? `input-${Math.random().toString(36).slice(2, 9)}`);
</script>

<div class="field" class:has-error={!!error}>
  {#if label}
    <label for={inputId} class="label">{label}</label>
  {/if}
  <input
    {id}
    {type}
    bind:value
    class="input"
    class:error-state={!!error}
    aria-describedby={error ? `${inputId}-error` : helperText ? `${inputId}-helper` : undefined}
    aria-invalid={error ? true : undefined}
  />
  {#if error}
    <span id="{inputId}-error" class="helper-text error-text" role="alert">{error}</span>
  {:else if helperText}
    <span id="{inputId}-helper" class="helper-text">{helperText}</span>
  {/if}
</div>

<style>
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
  }

  .label {
    font-family: var(--font-body);
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--on-surface-variant);
  }

  .input {
    background-color: var(--surface-zone);
    border: 1.5px solid transparent;
    border-radius: 0.5rem;
    padding: 0.625rem 0.875rem;
    font-family: var(--font-body);
    font-size: 0.9375rem;
    color: var(--on-surface);
    outline: none;
    transition: background-color var(--motion-standard), border-color var(--motion-standard);
  }

  .input:focus {
    background-color: var(--surface-card);
    border-color: var(--primary);
  }

  .input.error-state {
    border-color: var(--error);
  }

  .helper-text {
    font-family: var(--font-body);
    font-size: 0.8125rem;
    color: var(--on-surface-variant);
  }

  .error-text {
    color: var(--error);
  }
</style>
