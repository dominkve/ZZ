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
    import OutputWindow from './OutputWindow.svelte';

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
    const maxLines = 30;
    const lineHeight = 24;

    const fixedHeight = EditorView.theme({
        "&": {
            minHeight: `${minLines * lineHeight}px`,
            maxHeight: `${maxLines * lineHeight}px`,
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
        if (sessionStorage.getItem("language")) {
            editor.dispatch({
                effects: lang.reconfigure(getLanguageExtension(language))
            });
        };
        console.log(editorState.doc.toString());
        // Cleanup function: Destroy CodeMirror when the component is removed
        return () => {
            editor.destroy();
        };
    });

    function setLang() {
        console.log(language);
        editor.dispatch({
            effects: lang.reconfigure(getLanguageExtension(language))
        });
    }

    let code = $state("");
    let browser = $state();
    function run() {
        code = get(editorContent);
        if (language == "html" || language == "javascript") {
            browser = true;
            code = get(editorContent);
            console.log(code);
        }
    }
</script>

<div class="container">
{#if select}
<select bind:value={language} onchange={setLang}>
    <option value="cpp">C++</option>
    <option value="javascript">JavaScript</option>
    <option value="html">HTML</option>
</select>
{/if}

<button class="run p-2 bg-blue-300 border-b-black m-2"
    onclick={run}>
    RUN
</button>

<div class="editor-container" bind:this={editorContainer}></div>

</div>
<div class="output-window">
    {#if browser}
    <iframe title="Output Window" srcdoc={code}></iframe>
    {/if}
</div>

<style>
    .output-window {
        position: absolute;
        display: flex;
        top: 92px;
        right: 25px;
        width: 40vw;
        height: 80vh;
        background-color: gray;
        border: 2px solid black;
    }

    .output-window iframe {
        flex: 1;
        border: none;
        background-color: white;
    }
    .container {
        position: absolute;
        top: 90px;
        left: 15px;
        width: 40vw;
    }

    .editor-container {
        width: 40vw; /* Adjust as needed */
        border: 2px solid white;
    }

    .run {
        left: 100px;
    }
    .run:hover {
        background: blue;
    }
</style>