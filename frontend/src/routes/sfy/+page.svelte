<script lang="ts">
    import CodeMirror from "$lib/components/CodeMirror.svelte";
	import { get } from "svelte/store";
    import { editorContent } from "../../editorStores";
    import { onDestroy, onMount } from 'svelte';
	import OutputWindow from "$lib/components/OutputWindow.svelte";

    let language: string = "";
    onMount(() => {
        let content = get(editorContent);

        if (content != "Start content") {
            sessionStorage.setItem("content", content);
        } else {
            content = sessionStorage.getItem("content") || "";
            editorContent.set(content);
        }
        language = sessionStorage.getItem('language') || "";

        const handleBeforeUnload  = () => {
            sessionStorage.setItem("content", get(editorContent));
        };

        window.addEventListener('beforeunload', handleBeforeUnload);
        
        onDestroy(() => {
            window.removeEventListener('beforeunload', handleBeforeUnload);
        });
    });

</script>

{#if language}
<CodeMirror {language}/>
{/if}
<OutputWindow />