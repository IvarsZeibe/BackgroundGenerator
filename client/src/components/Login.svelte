<script lang="ts">
    import backendService from '../services/backend-service';

    export let currentRoute;
    export let params;

    let email = '';
    let password = '';

    function handleSubmit(e) {
        e.preventDefault();
        backendService.login(email, password).then(res => {
            if (!res.ok) {
                res.text().then(data => console.log(data));
            }
        }).catch(e => console.log(e));
    }

</script>

<div>
    <form on:submit={handleSubmit}>
        <input type="text" name="email" placeholder="Email" bind:value={email} />
        <input type="password" name="password" placeholder="Password" bind:value={password} />
        <button>Login</button>
    </form>
</div>