<script lang="ts">
    import { onMount } from 'svelte';
    import { get } from "svelte/store";
    import { editorContent } from "../../stores";
    import Navbar from '$lib/components/Navbar.svelte';
    import CodeMirror from '$lib/components/CodeMirror.svelte';
	import ExecuteButton from '$lib/components/buttons/ExecuteButton.svelte';

    let lang = $state();
    let languages: { language: String }[] = $state([]);
    onMount(async () => {
        console.log('Fetching languages.');
        const response = await fetch("http://localhost:3000/languages", {
            method: "GET",
        });

        const data = await response.json();
        console.log("Fetched:\n", data);
        languages = data;
    });

    async function run() {
        let content = get(editorContent);
        console.log("Editor content:\n", content);
        console.log("Language: ", lang);

        fetch("http://localhost:3000/execute", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ 
                language: lang,
                code: content,
            }),
        })
        .then(response => response.json())
        .then(data => console.log(data))
    }
</script>
<Navbar />
<CodeMirror />
<ExecuteButton run={run}/>
<select bind:value={lang}>
    {#each languages as { language }}
    <option>{language}</option>
    {/each}
</select>

