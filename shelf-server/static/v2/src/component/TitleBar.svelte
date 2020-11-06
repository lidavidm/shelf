<script context="module">
    import { writable } from "svelte/store";
    export let store = writable("Home");
</script>

<script>
</script>

<style>
    header {
        border-bottom: 1px solid #000;
        display: flex;
        flex-direction: row-reverse;
        justify-content: space-between;
        margin-bottom: 1em;
    }

    h1,
    h2 {
        font-size: 2em;
        margin: 0;
        padding: 0;
    }
</style>

<header>
    <h1>Shelf</h1>

    <h2>
        <slot />
    </h2>
</header>
