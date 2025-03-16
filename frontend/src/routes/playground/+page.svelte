<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { get } from "svelte/store";
    import { editorContent } from "../../editorStores";
    import CodeMirror from '$lib/components/CodeMirror.svelte';
	import ExecuteButton from '$lib/components/buttons/ExecuteButton.svelte';

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
</script>

{#if editor}
<CodeMirror select={true}/>
{/if}


<!--
<ExecuteButton run={run}/> -->



