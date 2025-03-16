<script lang="ts">
    import NextButton from "$lib/components/buttons/NextButton.svelte";
</script>
<h2>Intro</h2>

<div class="button-container">
    <NextButton destination="/courses/html/hello_world"/>
</div>

<style>
    .button-container {
        justify-content: end;
    }
</style>