<script>
    import router from "page";
    import EditItem from "./EditItem.svelte";
    import Shelf from "./Shelf.svelte";
    import Toast from "./component/Toast.svelte";

    let page = Shelf;
    let params;
    let content;

    router("*", function (ctx, next) {
        if (ctx.init) {
            next();
        } else {
            content.classList.add("page-transition");
            setTimeout(function () {
                content.classList.remove("page-transition");
                next();
            }, 100);
        }
    });

    router("/", () => (page = Shelf));
    router(
        "/edit/:key",
        (ctx, next) => {
            params = ctx.params;
            next();
        },
        () => (page = EditItem)
    );

    router.start({ hashbang: true });
</script>

<style>
    #content {
        transition: opacity 100ms ease-in-out;
    }

    :global(.page-transition) {
        opacity: 0;
    }
</style>

<div>
    <div id="content" bind:this={content}>
        <svelte:component this={page} {router} {params} />
    </div>
    <Toast />
</div>
