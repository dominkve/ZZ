<script lang="ts">
    import { onMount } from "svelte";
    import { EditorView, basicSetup } from "codemirror";
    import { EditorState } from "@codemirror/state";
    import { cpp } from "@codemirror/lang-cpp";

    export let content: string; // The content of the code snippet
    export let language:string;

    let snippetContainer: HTMLDivElement;
    let editor: EditorView;
    
    // Function to select the right language extension
    function getLanguageExtension(language: string) {
        const languages: Record<string, any> = {
            cpp: cpp(),
        };
        return languages[language] || [];
    }

    onMount(() => {
        editor = new EditorView({
            doc: content,
            parent: snippetContainer,
            extensions: [
            basicSetup,
            getLanguageExtension(language),
            EditorView.theme({
                "&": {
                    fontSize: "18px" // Set font size directly here
                }
            }),
            EditorState.readOnly.of(true),
            EditorView.editable.of(false),
            EditorView.contentAttributes.of({tabindex: "0"})
        ]
        });

        return () => {
            editor.destroy();
        };
    });
</script>

<div bind:this={snippetContainer}></div> 