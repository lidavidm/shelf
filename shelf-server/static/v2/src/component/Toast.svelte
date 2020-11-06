<script>
    import { fly } from "svelte/transition";
    import toasts from "./toast.js";
</script>

<style>
    #toasts {
        position: fixed;
        left: 0;
        right: 0;
        bottom: 0;
    }

    .toast {
        background: var(--theme-3);
        box-shadow: 0 2px 5px #000;
        color: #fff;
        font-size: 1.25em;
        margin: 1em auto;
        min-width: 20em;
        padding: 0.5em 1em;
        width: max-content;
    }
    .toast p {
        margin: 0;
    }
</style>

<div id="toasts">
    {#each $toasts as toast (toast.id)}
        <div class="toast" transition:fly={{ y: 200, duration: 200 }}>
            <p><em>{toast.title}</em> {toast.body}</p>
        </div>
    {/each}
</div>
