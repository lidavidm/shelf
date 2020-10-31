<script>
    import router from "page";
    import EditItem from "./EditItem.svelte";
    import Shelf from "./Shelf.svelte";
    import Toast from "./component/Toast.svelte";

    let page = Shelf;
    let params;

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
</style>

<div>
    <h1>shelf</h1>
    <svelte:component this={page} {router} {params} />
    <Toast />
</div>
