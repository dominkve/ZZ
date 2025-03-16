<script lang="ts">
    import CodeSnippet from "$lib/components/CodeSnippet.svelte";
    const language = "html";
</script>
<h2>Hello, World!</h2>
<p>The traditional way of starting.</p>

<CodeSnippet {language} content={
`<h1>Hello, World!<h1>`
}/>