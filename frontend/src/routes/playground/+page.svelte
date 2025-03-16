<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { get } from "svelte/store";
    import { editorContent } from "../../editorStores";
    import CodeMirror from '$lib/components/CodeMirror.svelte';
	import ExecuteButton from '$lib/components/buttons/ExecuteButton.svelte';
    import OutputWindow from '$lib/components/OutputWindow.svelte';

    let editor: boolean = false;
    onMount(() => {

        let content = get(editorContent);

        if (content != "Start content") {
            sessionStorage.setItem("content", content);
        } else {
            content = sessionStorage.getItem("content") || "";
            editorContent.set(content);
        }

        editor = true;
        const handleBeforeUnload  = () => {
            sessionStorage.setItem("content", get(editorContent));
        };

        window.addEventListener('beforeunload', handleBeforeUnload);
        
        onDestroy(() => {
            window.removeEventListener('beforeunload', handleBeforeUnload);
        });
    });
/*
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
    } */
</script>

{#if editor}
<CodeMirror select={true}/>
{/if}


<!--
<ExecuteButton run={run}/> -->



