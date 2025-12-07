<script lang="ts">
  // Sample notes data
  const notes = $state([
    {
      id: 1,
      title: 'Getting Started',
      preview: 'Welcome to your knowledge base! Start by creating your first note...',
      tags: ['onboarding'],
    },
    {
      id: 2,
      title: 'Project Ideas',
      preview:
        'Brainstorm: Build a personal wiki, integrate with external tools, add graph visualization...',
      tags: ['ideas', 'project'],
    },
    {
      id: 3,
      title: 'Learning Resources',
      preview: 'Collection of useful links and references for learning new technologies...',
      tags: ['learning', 'resources'],
    },
  ]);
</script>

<div class="notes-page">
  <div class="header">
    <h2>My Notes</h2>
    <button class="create-btn">+ New Note</button>
  </div>

  <div class="notes-grid">
    {#each notes as note (note.id)}
      <div class="note-card">
        <h3>{note.title}</h3>
        <p class="preview">{note.preview}</p>
        <div class="tags">
          {#each note.tags as tag}
            <span class="tag">{tag}</span>
          {/each}
        </div>
      </div>
    {/each}
  </div>

  <div class="empty-state">
    <p>This is a placeholder notes view. In the full implementation:</p>
    <ul>
      <li>Create and edit notes with markdown support</li>
      <li>Bidirectional links between notes</li>
      <li>Tag-based organization</li>
      <li>Full-text and semantic search</li>
      <li>Graph visualization of connections</li>
    </ul>
  </div>
</div>

<style>
  .notes-page {
    max-width: 1000px;
    margin: 0 auto;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  h2 {
    font-size: 2rem;
    background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .create-btn {
    padding: 0.75rem 1.5rem;
    background: linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%);
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .create-btn:hover {
    opacity: 0.9;
  }

  .notes-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .note-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    padding: 1.5rem;
    cursor: pointer;
    transition: all 0.2s;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .note-card:hover {
    border-color: #4f46e5;
    box-shadow: 0 4px 6px rgba(79, 70, 229, 0.1);
  }

  .note-card h3 {
    margin-bottom: 0.75rem;
    color: #111827;
    font-size: 1.25rem;
  }

  .preview {
    color: #6b7280;
    font-size: 0.95rem;
    margin-bottom: 1rem;
    line-height: 1.5;
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .tag {
    padding: 0.25rem 0.75rem;
    background: #ede9fe;
    color: #7c3aed;
    border-radius: 9999px;
    font-size: 0.875rem;
    font-weight: 500;
  }

  .empty-state {
    background: #f9fafb;
    border: 1px dashed #d1d5db;
    border-radius: 0.5rem;
    padding: 2rem;
  }

  .empty-state p {
    color: #374151;
    font-weight: 500;
    margin-bottom: 1rem;
  }

  .empty-state ul {
    list-style-position: inside;
    color: #6b7280;
  }

  .empty-state li {
    margin-bottom: 0.5rem;
  }
</style>
