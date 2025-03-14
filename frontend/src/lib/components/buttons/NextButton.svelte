<script lang="ts">
    export let destination: string; // Receive destination as a prop
</script>


<button onclick={() => (window.location.href = destination)} class="next">NEXT</button>


<style>
    .next {
        background-color: lightblue;
        color: black;
        border-radius: 25%;
        padding: 1%;
        margin: 1%;
    }

    .next:hover {
        background-color: green;
    }
</style>