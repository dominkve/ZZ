<script lang="ts">
    export let run; // Receive function as a prop
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