<script lang="ts">
    import { onMount } from 'svelte';

    const language = "html";
    onMount(() => {
        sessionStorage.setItem('language', language);
    });
</script>

<div>
    <h1><a href="/courses/html">HTML</a></h1>
    <slot></slot>
</div>