<script lang="ts">
	const { data } = $props();
</script>

<div class="p-8 font-mono">
	{#if data.error}
		<p class="text-red-500">Error: {data.error}</p>
	{:else}
		<h1 class="text-2xl font-bold mb-4">Login successful</h1>
		<p class="text-sm text-gray-500 mb-2">access_token (dev display only):</p>
		<pre class="bg-gray-100 p-4 rounded text-xs break-all">{data.access_token}</pre>
	{/if}
	<a href="/" class="mt-4 block text-blue-500 underline">Back to home</a>
</div>
