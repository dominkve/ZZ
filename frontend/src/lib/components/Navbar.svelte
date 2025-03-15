<div class="navbar">
    <!-- 早 -->
    <h1>Zaoζάω</h1>
    <nav>
        <a href="/">Home</a>
        <a href="/problems">Problems</a>
        <a href="/playground">Playground</a>
        <a href="/courses">Courses</a>
    </nav>
</div>

<style>
    .navbar {
        background-color: gray;
        width: 100vw;
        height: 70px;
        padding: 30px;
        display: flex;
        align-items: center;
        justify-content: flex-start;
        gap: 20px;
        position: fixed;
        z-index: 1000;
        top: 0;
        left: 0;
    }

    .navbar h1 {
        font-size: 30px;
        color: rgb(71, 31, 16);
    }

    .navbar nav {
        display: flex;
        gap: 15px;
    }

    .navbar a {
        font-size: 25px;
        font-weight: bold;
        line-height: 15px;
        margin-right: 15px;
    }

    .navbar a:hover {
        font-size: 27px;
        color: white;
    }
</style>