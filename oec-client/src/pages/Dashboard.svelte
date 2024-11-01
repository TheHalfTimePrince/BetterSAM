<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Card, CardContent, CardHeader, CardTitle } from "$lib/components/ui/card";
  import { ClipboardCopy } from "lucide-svelte";
  import { onMount, onDestroy } from 'svelte';

  let streamLink: string | null = null;
  let sessionId: string | null = null;
  let ws: WebSocket | null = null;

  // Initialize WebSocket connection
  function initWebSocket() {
    ws = new WebSocket('ws://localhost:8080/ws'); // Adjust URL to match your Rust server
    
    ws.onopen = () => {
      console.log('Connected to WebSocket server');
    };

    ws.onmessage = async (event) => {
      console.log('Received WebSocket message:', event.data);
      const data = JSON.parse(event.data);
      if (data.type === 'StreamLink') {
        // Get the full URL including the current domain
        const baseUrl = window.location.origin;
        // Insert the hash before the query parameters
        streamLink = `${baseUrl}/#${data.url}`;
        sessionId = data.session_id;
        console.log('Stream link generated:', streamLink);
      }
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    ws.onclose = () => {
      console.log('Disconnected from WebSocket server');
      ws = null;
    };
  }

  onMount(() => {
    initWebSocket();
  });

  onDestroy(() => {
    if (ws) {
      ws.close();
    }
  });

  function generateStreamLink() {
    console.log('Generating stream link');

    if (ws && ws.readyState === WebSocket.OPEN) {
        const message = {
            type: 'GenerateStream'
        };
        console.log('Sending message:', message);
        ws.send(JSON.stringify(message));
    } else {
        console.error('WebSocket is not connected');
        initWebSocket();
    }
  }

  function copyToClipboard() {
    if (streamLink) {
      navigator.clipboard.writeText(streamLink);
    }
  }
</script>

<div class="container mx-auto p-4 max-w-4xl">
  <header class="text-center mb-8 bg-primary text-primary-foreground p-6">
    <h1 class="text-3xl font-bold mb-2">OpenEmergencyConnect</h1>
    <p class="text-lg">
      An open-source alternative to GoodSAM's emergency services platform.
    </p>
  </header>
  
  <div class="space-y-6">
    <Card class="w-full border-primary/20">
      <CardHeader class="border-b border-primary/10">
        <CardTitle class="text-primary text-xl">Video Stream</CardTitle>
      </CardHeader>
      <CardContent class="p-0">
        <div class="aspect-video bg-gray-100 flex items-center justify-center">
          {#if streamLink}
            <video class="w-full h-full object-cover" controls>
              <source src={streamLink} type="video/mp4" />
              Your browser does not support the video tag.
            </video>
          {:else}
            <p class="text-gray-500">No active stream</p>
          {/if}
        </div>
      </CardContent>
    </Card>

    <Card class="border-primary/20">
      <CardHeader class="border-b border-primary/10">
        <CardTitle class="text-primary text-xl">Stream Controls</CardTitle>
      </CardHeader>
      <CardContent>
        <Button 
          on:click={generateStreamLink} 
          class="w-full mb-4 bg-primary hover:bg-primary/90 text-primary-foreground"
        >
          Generate Streaming Link
        </Button>
        
        {#if streamLink}
          <div class="mt-4">
            <label for="streamLink" class="block text-sm font-medium text-primary mb-2">
              Share this link with emergency services to allow them to stream to you:
            </label>
            <div class="flex">
              <Input
                type="text"
                id="streamLink"
                value={streamLink}
                readonly
                class="flex-grow border-primary/20"
              />
              <Button
                on:click={copyToClipboard}
                class="ml-2 border-primary hover:bg-primary hover:text-primary-foreground"
                variant="outline"
                title="Copy to clipboard"
              >
                <ClipboardCopy class="h-4 w-4" />
                <span class="sr-only">Copy to clipboard</span>
              </Button>
            </div>
          </div>
        {/if}
      </CardContent>
    </Card>
  </div>
</div>