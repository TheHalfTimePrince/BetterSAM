<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { push } from 'svelte-spa-router';
  </script>
  
  <div class="container mx-auto p-4">
    <h1 class="text-3xl font-bold mb-4">Welcome to OpenEmergencyConnect</h1>
    <Button on:click={() => push('/dashboard')}>Go to Dashboard</Button>
  </div>