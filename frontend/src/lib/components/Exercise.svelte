<script lang="ts">
    export let exercise: { "question": string, "answer": string };

    let user_answer: string = "Answer here";
    let correct: boolean = false;

    function checkAnswer() {
        correct = user_answer === exercise.answer;
    }
</script>

<div class="exercise">
    <h1>{exercise.question}</h1>
    <input type="text" bind:value={user_answer} />
    <button on:click={checkAnswer}>Submit</button>
    <p>{correct ? "Correct!": "Try again."}</p>
</div>

<style>
    .exercise {
        padding: 20px;
        margin: 20px;
        background-color: lightgray;
        border: 2px solid black;
    }

    .exercise h1 {
        font-size: 20px;
        font-weight: bold;
        padding: 10px;
    }

    .exercise input {
        width: 80%;
        padding: 1%;
        margin: 1%;
    }

    .exercise button {
        background-color: lightgreen;
        color: black;
        border-radius: 20%;
        padding: 1%;
        margin: 1%;
    }

    .exercise button:hover {
        background-color: green;
        color: white;
    }

    .exercise p {
        background-color: lightblue;
        font-size: 18px;
        font-weight: bold;
        padding: 10px;
    }
</style>