<script lang="ts">
    import NextButton from '$lib/components/buttons/NextButton.svelte';
    import PreviousButton from '$lib/components/buttons/PreviousButton.svelte';
    import CodeSnippet from '$lib/components/CodeSnippet.svelte';
</script>


<h1>C++</h1>
<h2>Hello, World</h2>

<CodeSnippet language="cpp" content={
`#include <iostream>

int main() {
    std::cout << "Hello, World!";
    return 0;
}`
} />

// I can also write without starting slash for relative path instead of absolute path
<div class="button-container">
    <PreviousButton destination="/courses/cpp" />
    <NextButton destination="/courses/cpp/variables" /> 
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