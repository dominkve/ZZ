<script lang="ts">
    import { page } from '$app/state';
    import { onMount } from 'svelte';
    import Navbar from '$lib/components/Navbar.svelte';

    let problem: {id: number, name: String, description: String } = { id: 0, name: "", description: "" };

    // Get problem ID from URL
    $: problem_name = page.params.name;
    console.log(problem_name)

    onMount(async () => {
        const response = await fetch(`http://localhost:3000/problems/${problem_name}`);
        problem = await response.json();
    });
</script>

<Navbar />
{#if problem}
    <h1>{problem.id}. {problem.name}</h1>
    <p>{problem.description}</p>
{:else}
    <p>Loading...</p>
{/if}