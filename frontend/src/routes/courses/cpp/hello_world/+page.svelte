<script lang="ts">
    import NextButton from '$lib/components/buttons/NextButton.svelte';
    import PreviousButton from '$lib/components/buttons/PreviousButton.svelte';
    import CodeSnippet from '$lib/components/CodeSnippet.svelte';
    import Exercise from '$lib/components/Exercise.svelte';

	const language = "cpp";

</script>


<h1>C++</h1>
<h2>Hello, World</h2>

<CodeSnippet {language} content={
`#include <iostream>

int main() {
    std::cout << "Hello, World!";
    return 0;
}`
}/>

<p>Explanation paragraph. This code snippet prints out hello, world.
    The <code>std::cout</code> is used to print out the string <code>"Hello, World!"</code> to the console.
    The <code>return 0;</code> statement is used to indicate that the program has executed successfully.
</p>

<Exercise exercise={{
    "question": "What is the output of the code snippet?",
    "answer": "Hello, World!",
}}/>

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