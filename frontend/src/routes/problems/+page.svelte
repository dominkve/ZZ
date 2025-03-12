<script lang="ts">
    import { onMount } from "svelte";
    import Navbar from '$lib/components/Navbar.svelte';

    let problems: { id: number; name: string }[] = [];

    onMount(async () => {
        const response = await fetch("http://localhost:3000/problems", {
            method: "GET",
        });
        const data = await response.json();
        console.log(data)
        problems = data;
    });
</script>
<Navbar />
<ul>
    <li class="list-head">Problem List</li>
    {#each problems as { id, name }}
        <li><a href={`/problems/${name}`}>{id}. {name}</a></li>
    {/each}
</ul>

<style>
    ul {
        width: 600px;
        display: grid;
        place-items: center;
        margin: 20px;
        font-size: 18px;
        background: lightgoldenrodyellow;
        padding: 20px;
        border-radius: 5%;
    }

    .list-head {
        font-size: 24px;
        margin-top: 5px;
        margin-bottom: 10px;
        margin-left: 5px;
        font-weight: bold;
        color: rgb(199, 84, 30);
    }
    li {
        width: 400px;
        text-align: left;
    }
</style>