<script>
    import logo from '../assets/svelte.svg';
    import logo2 from '../assets/vite.svg';
    import { navigateTo, Navigate } from 'svelte-router-spa';

    let generators = [{name: "triangle"}, {name: "colorful"}];
    
    export let currentRoute;
    console.log(currentRoute);
</script>

<div>
    <!-- <img src={logo2} alt="nothing" />
    <img src={logo} alt="nothing" />
    Hello there! -->
    <h1>Background generator</h1>
    <h2>Choose a generator:</h2>
    <ul>
        {#each generators as generator}
        <li>
            <!-- //`generator?name=${generator.name}` -->
            <button on:click={() => {navigateTo(`generator?name=${generator.name}`)}}>{generator.name}</button>
        </li>
        {/each}
    </ul>
</div>

<style>
h1 {
    font-size: 60px;
    text-align: center;
}
ul {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}
li {
    list-style-type: none;
    margin: 20px;
}
li button {
    display: block;
    background-color: gray;
    width: 500px;
    height: 200px;
}
li button:hover {
    cursor: pointer;
}
</style>