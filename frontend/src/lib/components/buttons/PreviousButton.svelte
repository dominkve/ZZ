<script lang="ts">
    export let destination: string; // Receive destination as a prop
</script>


<button onclick={() => (window.location.href = destination)} class="previous">BACK</button>



<style>

    .previous {
        background-color: lightblue;
        color: black;
        border-radius: 20%;
        padding: 2%;
        margin: 2%;
        font-weight: bold;
    }

    .previous:hover {
        background-color: green;
    }
</style>