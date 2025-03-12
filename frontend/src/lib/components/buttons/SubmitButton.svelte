<script lang="ts">
    import { get } from "svelte/store";
    import { editorContent } from "../../../stores";

    async function submit() {
        const content = get(editorContent); // Get store value without subscribing
        console.log("Submitting content:\n", content);

        fetch("http://localhost:3000/submit", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ message: content })
        })
        .then(response => response.json())
        .then(data => console.log("Response:", data))
        .catch(error => console.error("Fetch error:", error));
    } 
</script>

<button onclick={submit} class="submit">SUBMIT</button>

<style>
    .submit {
        background-color: lightblue;
        color: black;
        border-radius: 20%;
        padding: 1%;
        margin: 1%
    }

    .submit:hover {
        background-color: green;
    }
</style>