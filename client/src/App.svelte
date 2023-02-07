<script>
    import { Router } from 'svelte-router-spa'
    import { routes } from './routes'
</script>

<div id="appContent">
    <Router {routes} />
</div>

<style>
#appContent {
    display: flex;
    flex-direction:column;
    min-height: 100vh;
}
</style>