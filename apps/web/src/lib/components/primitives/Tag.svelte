<script lang="ts">
  interface Props {
    label: string;
    onremove?: () => void;
  }

  let { label, onremove }: Props = $props();
</script>

<span class="tag">
  <span class="tag-label">{label}</span>
  {#if onremove}
    <button
      class="remove-btn"
      onclick={onremove}
      aria-label="Remove {label}"
      type="button"
    >
      ×
    </button>
  {/if}
</span>

<style>
  .tag {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.625rem;
    border-radius: 9999px;
    background-color: var(--primary-container);
    color: var(--primary);
    font-family: var(--font-body);
    font-size: 0.8125rem;
    font-weight: 500;
  }

  .tag-label {
    line-height: 1.2;
  }

  .remove-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1rem;
    height: 1rem;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--primary);
    font-size: 1rem;
    line-height: 1;
    padding: 0;
    border-radius: 50%;
    transition: background-color var(--motion-standard);
  }

  .remove-btn:hover {
    background-color: color-mix(in srgb, var(--primary) 15%, transparent);
  }
</style>
