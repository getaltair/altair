<script lang="ts">
  interface Props {
    label: string;
    color?: string;
  }

  let { label, color }: Props = $props();
</script>

<span class="badge" style={color ? `background-color: ${color}` : undefined}>
  {label}
</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    padding: 0.125rem 0.625rem;
    border-radius: 9999px;
    background-color: var(--surface-highest);
    color: var(--on-surface);
    font-family: var(--font-body);
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    white-space: nowrap;
  }
</style>
