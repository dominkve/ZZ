<script lang="ts">
    import NextButton from '$lib/components/buttons/NextButton.svelte';
</script>

<h1>C++</h1>
<h2>Intro</h2>

<div class="button-container">
    <NextButton destination="/courses/cpp/hello_world" />
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
        justify-content: end;
        width: 100%;
    }
</style>