<script lang="ts">
        import { onMount } from 'svelte';

const language="cpp";
onMount(() => {
    sessionStorage.setItem('language', language);
});
</script>

<div>
    <h1><a href="/courses/cpp">C++</a></h1>
    <slot></slot>
</div>