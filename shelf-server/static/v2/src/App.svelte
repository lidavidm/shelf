<script>
    import router from "page"
    import EditItem from './EditItem.svelte'
    import Shelf from './Shelf.svelte'

    let page = Shelf;
    let params;

    router('/', () => page = Shelf)
    router('/edit/:key', (ctx, next) => {
        params = ctx.params;
        next();
    }, () => page = EditItem)

    router.start({ hashbang: true })
</script>

<div>
    <h1>shelf</h1>
    <svelte:component this={page} router={router} params={params} />
</div>

<style>
</style>
