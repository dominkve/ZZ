<script lang="ts">
    import NextButton from '$lib/components/buttons/NextButton.svelte';
</script>

<h2>Intro</h2>
<p>C++ is a compiled language that has a lot of uses, its fast and powerfull.
    It's used in algorithms, database engines, brokers, systems engineering, embeeded systems, cars...
</p>
<div class="button-container">
    <NextButton destination="/courses/cpp/hello_world" />
</div>

<style>
    .button-container {
        justify-content: end;
    }
</style>