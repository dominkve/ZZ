<script lang="ts">
    import NextButton from '$lib/components/buttons/NextButton.svelte';
    import PreviousButton from '$lib/components/buttons/PreviousButton.svelte';
</script>


<h1>C++</h1>
<h2>Functions</h2>

<div class="button-container">
    <PreviousButton destination="/courses/cpp/variables" />
</div>


<style>
    h1 {
        text-align: center;
        font-size: 40px;
        font-weight: bold;
        padding: 20px;
        color: gainsboro;
    }

    h2 {
        text-align: left;
        font-size: 35px;
        padding: 15px;
    }

    .button-container {
        display: flex;
        justify-content: space-between;
        width: 100%;
    }
</style>