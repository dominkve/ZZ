<script>
    import '../courses/courses.css';
	import CoursesNav from "$lib/components/CoursesNav.svelte";

</script>
<div class="courses">
    <CoursesNav />
    <main>
        <slot></slot>
    </main>
</div>

<style>
    main {
        margin-left: 30vw;
        margin-right: 30vw;
    }
</style>