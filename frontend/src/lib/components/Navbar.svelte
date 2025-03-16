<div class="navbar">
    <!-- 早 -->
    <nav>
        <a href="/" class="name">Zaoζάω</a>
        <a href="/courses">Courses</a>
        <a href="/problems">Problems</a>
        <a href="/playground">Playground</a>
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



    .navbar nav {
        display: flex;
        gap: 15px;
        margin-left: 20vw;
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

    .navbar a.name {
        font-size: 30px;
        color: rgb(71, 31, 16);
    }

    .navbar a.name:hover {
        color: rgb(71, 31, 16);
    }
</style>