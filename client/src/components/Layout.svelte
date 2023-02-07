<script lang="ts">
    import { Route } from 'svelte-router-spa';
    import NavBar from './NavBar.svelte';
    import Footer from './Footer.svelte';
    import logo from '../assets/vite.svg';
    
    export let currentRoute;
    export let params;
</script>

<svelte:head>
    <link rel="icon" type="image/svg+xml" href={logo} />
</svelte:head>

<NavBar />
<main>
    <Route {currentRoute} {params} />
</main>
<div id="footer">
    <Footer />
</div>

<style>
main {
    flex: 1
}
#footer {
    min-height:50px;
}
</style>