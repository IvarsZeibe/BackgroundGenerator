import PublicLayout from './components/Layout.svelte';
import Welcome from './components/Welcome.svelte';
import About from './components/About.svelte';
import Login from './components/Login.svelte';
import SignUp from './components/SignUp.svelte';
import Users from './components/Users.svelte';
import Generator from './components/Generator.svelte';

function userIsAdmin() {
  //check if user is admin and returns true or false
}

const routes = [
  { name: '/', component: Welcome, layout: PublicLayout},
  { name: 'about', component: About, layout: PublicLayout },
  { name: 'login', component: Login, layout: PublicLayout },
  { name: 'signup', component: SignUp, layout: PublicLayout },
  { name: 'users', component: Users, layout: PublicLayout },
  { name: 'generator', component: Generator, layout: PublicLayout },
  { name: '404', redirectTo: '/' }
]

export { routes }