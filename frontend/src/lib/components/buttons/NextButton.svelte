<script lang="ts">
    export let destination: string; // Receive destination as a prop
</script>


<button onclick={() => (window.location.href = destination)} class="next">NEXT</button>


<style>
    .next {
        background-color: lightblue;
        color: black;
        border-radius: 25%;
        padding: 2%;
        margin: 2%;
        font-weight: bold;
    }

    .next:hover {
        background-color: green;
    }
</style>