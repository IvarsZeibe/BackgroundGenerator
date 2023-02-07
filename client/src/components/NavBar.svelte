<script lang="ts">
  import { Navigate } from 'svelte-router-spa';
</script>

<div id="navbar">
    <ul>
        <li><Navigate to="/" title="something">Home</Navigate>
        <li><Navigate to="about">About</Navigate>
        <li><Navigate to="login">Login</Navigate>
        <li><Navigate to="signup">Sign Up</Navigate>
        <li><Navigate to="users">View users</Navigate>
    </ul>
</div>

<style>
ul {
  list-style-type: none;
  margin: 0;
  padding: 0;
  overflow: hidden;
  background-color: #333;
}

li {
  float: left;
}

:global(#navbar li a) {
  display: block;
  color: white;
  text-align: center;
  padding: 14px 16px;
  text-decoration: none;
}

:global(#navbar li a:hover) {
  background-color: #111;
}
</style>