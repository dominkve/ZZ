<script lang="ts">
	import '../app.css';
	import Navbar from '$lib/components/Navbar.svelte';
	let { children } = $props();
</script>

<div class="main-layout">
    <Navbar />
    <main>
        {@render children()}
    </main>
</div>


