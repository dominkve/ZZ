<script lang="ts">
        import { onMount } from 'svelte';

const language="cpp";
onMount(() => {
    sessionStorage.setItem('language', language);
});
</script>

<div>
    <slot></slot>
</div>