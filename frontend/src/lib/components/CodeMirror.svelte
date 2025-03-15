<script lang="ts">
    import { onMount } from 'svelte';
    import { basicSetup } from 'codemirror'; // Just a collection of extensions
    import { EditorView, keymap } from '@codemirror/view'; // The UI of the editor
    import { editorContent } from "../../editorStores";
    import { indentWithTab } from '@codemirror/commands';
    import { oneDark } from '@codemirror/theme-one-dark';
    import { html } from '@codemirror/lang-html';
    import { javascript } from '@codemirror/lang-javascript';
    import { cpp } from '@codemirror/lang-cpp';
    import { get } from 'svelte/store';

    let { language = ""}: { language: string } = $props();

    function getLanguageExtension(lang: string) {
        const languages: Record<string, any> = {
            "cpp": cpp(),
            "javascript": javascript(),
            "html": html(),
        };
        return languages[lang] || [];
    }


    let editorContainer: HTMLDivElement;
    let editor: EditorView;

    const minLines = 15;
    const lineHeight = 24;

    const fixedHeight = EditorView.theme({
        "&": {
            minHeight: `${minLines * lineHeight}px`,
        },
        ".cm-scroller": {
            overflow: "auto",
        },
    }, {dark: true});

    onMount(() => {
        console.log('the component has mounted');
        
        // create the UI interface
        editor = new EditorView({
            doc: get(editorContent), // The content the user sees
            parent: editorContainer, 
            extensions: [
                basicSetup,
                keymap.of([indentWithTab]),
                oneDark,
                getLanguageExtension(language),
                fixedHeight,
                EditorView.theme({
                    "&": {
                        fontSize: "18px"
                    }
                }),
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) {
                        editorContent.set(editor.state.doc.toString()); // Update the store
                    }
                })
            ]
        });

        // Cleanup function: Destroy CodeMirror when the component is removed
        return () => {
            editor.destroy();
        };
    });


</script>

<div class="editor-container" bind:this={editorContainer}></div>

<style>
    .editor-container {
        position: absolute;
        top: 92px;
        left: 15px;
        width: 40vw; /* Adjust as needed */
        border: 2px solid white;
    }
</style>