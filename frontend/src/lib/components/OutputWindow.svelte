<script lang="ts">
    export let content;
    export let done;
</script>
<div class="output-window">
    <iframe title="Output Window" srcdoc={content}></iframe>
</div>

<style>
    .output-window {
        position: absolute;
        top: 92px;
        right: 25px;
        width: 40vw;
        height: 80vh;
        background-color: gray;
        border: 2px solid black;
    }
</style>