<script lang="ts">
    import backendService from '$lib/backend-service';

    let email = '';
    let password = '';

    function handleSubmit(e: { preventDefault: () => void; }) {
        e.preventDefault();
        backendService.login(email, password).then(res => {
            if (!res.ok) {
                res.text().then(data => console.log(data));
            } else {
                backendService.setProfile();
            }
        }).catch(e => console.log(e));
    }
</script>

<h1>Login</h1>
<div>
    <form on:submit={handleSubmit}>
        <input type="text" name="email" placeholder="Email" bind:value={email} />
        <input type="password" name="password" placeholder="Password" bind:value={password} />
        <button>Login</button>
    </form>
</div>