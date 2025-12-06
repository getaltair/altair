<script lang="ts">
  // Sample items data
  const items = $state([
    {
      id: 1,
      name: 'Laptop Charger',
      location: 'Office Desk',
      quantity: 1,
      tags: ['electronics'],
    },
    {
      id: 2,
      name: 'Winter Jacket',
      location: 'Bedroom Closet',
      quantity: 1,
      tags: ['clothing', 'seasonal'],
    },
    {
      id: 3,
      name: 'USB Cables',
      location: 'Drawer - Tech',
      quantity: 5,
      tags: ['electronics', 'cables'],
    },
  ]);
</script>

<div class="items-page">
  <div class="header">
    <h2>My Items</h2>
    <button class="create-btn">+ Add Item</button>
  </div>

  <div class="items-grid">
    {#each items as item (item.id)}
      <div class="item-card">
        <h3>{item.name}</h3>
        <div class="location">
          <span class="location-icon">📍</span>
          <span>{item.location}</span>
        </div>
        <p class="quantity">Quantity: {item.quantity}</p>
        <div class="tags">
          {#each item.tags as tag}
            <span class="tag">{tag}</span>
          {/each}
        </div>
      </div>
    {/each}
  </div>

  <div class="empty-state">
    <p>This is a placeholder items view. In the full implementation:</p>
    <ul>
      <li>Add items with photos, descriptions, and metadata</li>
      <li>Organize by location, category, or custom tags</li>
      <li>Track quantities and expiration dates</li>
      <li>Search and filter across all items</li>
      <li>Generate reports and inventory summaries</li>
    </ul>
  </div>
</div>

<style>
  .items-page {
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
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .create-btn {
    padding: 0.75rem 1.5rem;
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
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

  .items-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .item-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    padding: 1.5rem;
    cursor: pointer;
    transition: all 0.2s;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .item-card:hover {
    border-color: #10b981;
    box-shadow: 0 4px 6px rgba(16, 185, 129, 0.1);
  }

  .item-card h3 {
    margin-bottom: 0.75rem;
    color: #111827;
    font-size: 1.25rem;
  }

  .location {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #6b7280;
    font-size: 0.95rem;
    margin-bottom: 0.5rem;
  }

  .location-icon {
    font-size: 1rem;
  }

  .quantity {
    color: #6b7280;
    font-size: 0.9rem;
    margin-bottom: 1rem;
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .tag {
    padding: 0.25rem 0.75rem;
    background: #d1fae5;
    color: #059669;
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
