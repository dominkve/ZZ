<script lang="ts">
    import { get } from "svelte/store";
    import { editorContent } from "../../../editorStores";

    async function run() {
        let content = get(editorContent);
        console.log("Editor content:\n", content);

        fetch("http://localhost:3000/run", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ message: content }),
        })
        .then(response => response.json())
        .then(data => console.log(data))
    }
</script>

<button onclick={run} class="run">RUN</button>

<style>
    .run {
        background-color: lightblue;
        color: black;
        border-radius: 25%;
        padding: 1%;
        margin: 1%;
    }

    .run:hover {
        background-color: green;
    }
</style>