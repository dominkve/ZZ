<script lang="ts">
    import { onMount } from "svelte";
    import { EditorView, basicSetup } from "codemirror";
    import { EditorState } from "@codemirror/state";
    import { cpp } from "@codemirror/lang-cpp";
    import { javascript } from "@codemirror/lang-javascript";
    import { html } from '@codemirror/lang-html';
    import { editorContent } from "../../editorStores";
    import { oneDark } from "@codemirror/theme-one-dark";


    let { content = "Hello!", language = "" } = $props(); // The content of the code snippet

    let snippetContainer: HTMLDivElement;
    let editor: EditorView;
    
    // Function to select the right language extension
    function getLanguageExtension(language: string) {
        const languages: Record<string, any> = {
            cpp: cpp(),
            javascript: javascript(),
            html: html(),
        };
        return languages[language] || [];
    }

    onMount(() => {
        editor = new EditorView({
            doc: content,
            parent: snippetContainer,
            extensions: [
            basicSetup,
            oneDark,
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

    function updateStore() {
        editorContent.set(editor.state.doc.toString());
    }
</script>

<div class="code-snippet-container">
    <div class="code-snippet" bind:this={snippetContainer}></div> 
    <a href="/sfy" onclick={updateStore}>TRY!</a>
</div>


<style>
    .code-snippet-container {
        display: grid;
        margin: 20px;
        width: 40vw;
        border: 2px solid black;
        background-color: gray;
    }

    .code-snippet {
        margin: 5px;
        margin-bottom: 10px;
        border: 1px solid black;
    }
    .code-snippet-container a {
        margin: 10px;
        font-size: 25px;
        font-weight: bold;
        color: white;
        padding: 1px;
    }

    .code-snippet-container a:hover {
        color: black;
    }
</style>