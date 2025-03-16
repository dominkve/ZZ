<script lang="ts">
    import { onMount } from 'svelte';
    import { basicSetup } from 'codemirror'; // Just a collection of extensions
    import { EditorView, keymap } from '@codemirror/view'; // The UI of the editor
    import { EditorState, Compartment } from '@codemirror/state';
    import { editorContent } from "../../editorStores";
    import { indentWithTab } from '@codemirror/commands';
    import { oneDark } from '@codemirror/theme-one-dark';
    import { html } from '@codemirror/lang-html';
    import { javascript } from '@codemirror/lang-javascript';
    import { cpp } from '@codemirror/lang-cpp';
    import { get } from 'svelte/store';

    let { language = 'py', select = false }: { language?: string, select?: boolean } = $props();

    let lang = new Compartment;


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

    let editorState: EditorState;

    onMount(() => {
        console.log('the component has mounted');
        
        editorState = EditorState.create({
            doc: get(editorContent),
            extensions: [
                basicSetup,
                keymap.of([indentWithTab]),
                oneDark,
                lang.of(cpp()),
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
        
        // create the UI interface
        editor = new EditorView({
            doc: get(editorContent), // The content the user sees
            state: editorState,
            parent: editorContainer, 
        });

        console.log(editorState.doc.toString());
        // Cleanup function: Destroy CodeMirror when the component is removed
        return () => {
            editor.destroy();
        };
    });

    function setLang() {
        console.log(lang);
        editor.dispatch({
            effects: lang.reconfigure(getLanguageExtension(language))
        });
        
    }
</script>

<div class="editor-container" bind:this={editorContainer}></div>
{#if select}
<select bind:value={language} onchange={setLang}>
    <option value="cpp">C++</option>
    <option value="javascript">JavaScript</option>
    <option value="html">HTML</option>
</select>
{/if}
<style>
    .editor-container {
        position: absolute;
        top: 92px;
        left: 15px;
        width: 40vw; /* Adjust as needed */
        border: 2px solid white;
    }
</style>