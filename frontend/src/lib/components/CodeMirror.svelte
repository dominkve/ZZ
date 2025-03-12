<script lang="ts">
    import { onMount } from 'svelte';
    import { basicSetup } from 'codemirror'; // Just a collection of extensions
    import { EditorView } from '@codemirror/view'; // The UI of the editor
    import { editorContent } from "../../stores";

    let editorContainer: HTMLDivElement;
    let editor: EditorView;

    onMount(() => {
        console.log('the component has mounted');
        
        // create the UI interface
        editor = new EditorView({
            doc: "Start content", // The content the user sees
            parent: editorContainer, 
            extensions: [
                basicSetup,
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

<div bind:this={editorContainer}></div>