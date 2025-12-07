<script lang="ts">
  // Placeholder quest list using Svelte 5 runes
  let quests = $state([
    { id: 1, title: 'Set up development environment', energyCost: 3, status: 'completed' },
    { id: 2, title: 'Build Guidance app scaffold', energyCost: 5, status: 'in-progress' },
    { id: 3, title: 'Implement quest management', energyCost: 8, status: 'pending' },
  ]);
</script>

<div class="quests">
  <h2>Active Quests</h2>
  <p>Your current adventures in task management.</p>

  <div class="quest-list">
    {#each quests as quest (quest.id)}
      <div class="quest-card" data-status={quest.status}>
        <div class="quest-header">
          <h3>{quest.title}</h3>
          <span class="energy-badge">{quest.energyCost} ⚡</span>
        </div>
        <div class="quest-status">
          <span class="status-badge">{quest.status}</span>
        </div>
      </div>
    {/each}
  </div>

  <div class="empty-state">
    <p>Quest creation UI coming soon! 🎮</p>
  </div>
</div>

<style>
  .quests {
    max-width: 900px;
  }

  h2 {
    font-size: 2rem;
    margin-bottom: 0.5rem;
    color: #333;
  }

  .quest-list {
    margin-top: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .quest-card {
    background: white;
    border: 2px solid #e5e7eb;
    border-radius: 0.5rem;
    padding: 1.5rem;
    transition: all 0.2s;
  }

  .quest-card:hover {
    border-color: #667eea;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  }

  .quest-card[data-status='completed'] {
    opacity: 0.7;
    background: #f9fafb;
  }

  .quest-card[data-status='in-progress'] {
    border-color: #667eea;
  }

  .quest-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .quest-header h3 {
    margin: 0;
    font-size: 1.25rem;
    color: #333;
  }

  .energy-badge {
    background: #fef3c7;
    color: #92400e;
    padding: 0.25rem 0.75rem;
    border-radius: 9999px;
    font-weight: 600;
    font-size: 0.875rem;
  }

  .status-badge {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
  }

  .quest-card[data-status='completed'] .status-badge {
    background: #d1fae5;
    color: #065f46;
  }

  .quest-card[data-status='in-progress'] .status-badge {
    background: #dbeafe;
    color: #1e40af;
  }

  .quest-card[data-status='pending'] .status-badge {
    background: #f3f4f6;
    color: #6b7280;
  }

  .empty-state {
    margin-top: 2rem;
    padding: 2rem;
    text-align: center;
    background: #f9fafb;
    border-radius: 0.5rem;
    color: #6b7280;
  }
</style>
