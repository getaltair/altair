<script lang="ts">
  import { getName, getTauriVersion, getVersion } from '@tauri-apps/api/app';

  let appName = $state('');
  let appVersion = $state('');
  let tauriVersion = $state('');

  async function loadInfo() {
    appName = await getName();
    appVersion = await getVersion();
    tauriVersion = await getTauriVersion();
  }

  loadInfo();
</script>

<div class="about">
  <h2>About Tracking</h2>

  <div class="info-card">
    <h3>Application Information</h3>
    <div class="info-grid">
      <div class="info-item">
        <span class="label">Name:</span>
        <span class="value">{appName || 'Loading...'}</span>
      </div>
      <div class="info-item">
        <span class="label">Version:</span>
        <span class="value">{appVersion || 'Loading...'}</span>
      </div>
      <div class="info-item">
        <span class="label">Tauri:</span>
        <span class="value">{tauriVersion || 'Loading...'}</span>
      </div>
      <div class="info-item">
        <span class="label">Framework:</span>
        <span class="value">Svelte 5</span>
      </div>
    </div>
  </div>

  <div class="info-card">
    <h3>Technology Stack</h3>
    <ul class="tech-list">
      <li>
        <strong>Frontend:</strong> Svelte 5 with runes (modern reactive state management)
      </li>
      <li><strong>Desktop:</strong> Tauri 2.9.3 (Rust-based native app framework)</li>
      <li><strong>Database:</strong> SurrealDB (graph database with vector search)</li>
      <li><strong>Storage:</strong> S3-compatible object storage (Minio/Backblaze B2)</li>
      <li><strong>Embeddings:</strong> Local ONNX models for semantic search</li>
      <li><strong>Build Tool:</strong> Vite 6 with TypeScript</li>
    </ul>
  </div>

  <div class="info-card">
    <h3>Features (Planned)</h3>
    <ul class="features-list">
      <li>📦 Item cataloging with photos and descriptions</li>
      <li>📍 Location-based organization and tracking</li>
      <li>🔍 Hybrid search (full-text + semantic)</li>
      <li>🏷️ Tag-based categorization and filtering</li>
      <li>📊 Inventory reports and statistics</li>
      <li>📱 Cross-platform sync (desktop and mobile)</li>
      <li>🌙 Local-first architecture with offline support</li>
      <li>🔒 End-to-end encryption for cloud sync</li>
    </ul>
  </div>
</div>

<style>
  .about {
    max-width: 800px;
    margin: 0 auto;
  }

  h2 {
    font-size: 2rem;
    margin-bottom: 2rem;
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .info-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .info-card h3 {
    margin-bottom: 1rem;
    color: #111827;
    font-size: 1.25rem;
  }

  .info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .info-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .label {
    font-size: 0.875rem;
    color: #6b7280;
    font-weight: 500;
  }

  .value {
    color: #111827;
    font-weight: 600;
  }

  .tech-list,
  .features-list {
    list-style-position: inside;
    color: #374151;
  }

  .tech-list li,
  .features-list li {
    margin-bottom: 0.75rem;
    line-height: 1.6;
  }

  .tech-list strong {
    color: #10b981;
  }
</style>
