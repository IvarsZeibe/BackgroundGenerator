<script lang="ts">
    import backendService from '$lib/backend-service';

    let email = '';
    let password = '';

    function handleSubmit(e: { preventDefault: () => void; }) {
        e.preventDefault();
        backendService.register(email, password).then(res => console.log(res));
    }
</script>

<h1>Sign up</h1>
<div>
    <form on:submit={handleSubmit}>
        <input type="text" name="email" placeholder="Email" bind:value={email} />
        <input type="password" name="password" placeholder="Password" bind:value={password} />
        <button>Sign up</button>
    </form>
</div>