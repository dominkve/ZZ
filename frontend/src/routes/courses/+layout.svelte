<script>
    import Navbar from "$lib/components/Navbar.svelte";
	import CoursesNav from "$lib/components/CoursesNav.svelte";
</script>
<div class="courses-layout">
    <Navbar />
    <CoursesNav />
    <main>
        <slot></slot>
    </main>
</div>