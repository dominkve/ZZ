<script lang="ts">
    import { Hamburger } from 'svelte-hamburgers';

    let open = $state(false);
</script>


<div class="courses">


<Hamburger 
    bind:open
    type="collapse"
    title="Toggle nav links"
    ariaControls="nav"
/>

<nav>
    <a href="/courses/python">Python</a>
    <a href="/courses/html">HTML</a>
    <a href="/courses/css">CSS</a>
    <a href="/courses/cpp">C++</a>
</nav>
</div>


<style>
    .courses {
        background-color: darkslategrey;
        width: 100vw;
        height: 25px;
        padding: 22px;
        display: flex;
        align-items: center;
        justify-content: flex-start;
        gap: 20px;
        margin-top: 70px;
        position: fixed;
        z-index: 1000;
        top: 0;
        left: 0;
    }

    .courses nav {
        display: flex;
        gap: 15px;
    }

    .courses a {
        font-size: 20px;
        font-weight: bold;
        line-height: 15px;
        margin-right: 15px;
        color: white;
    }

    .courses a:hover {
        font-size: 22px;
        color: black;
    }
</style>