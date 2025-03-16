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