<script lang="ts">
  import Router from 'svelte-spa-router';
  import Home from './pages/Home.svelte';
  import Dashboard from './pages/Dashboard.svelte';
  import Sender from './pages/Sender.svelte';

  const routes = {
    '/': Home,
    '/dashboard': Dashboard,
    '/sender': Sender,
    '/sender?:query': Sender,
  };
</script>

<Router {routes} />