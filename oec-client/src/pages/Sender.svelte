<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { querystring } from "svelte-spa-router";
  
    let ws: WebSocket | null = null;
    let mediaRecorder: MediaRecorder | null = null;
    let videoElement: HTMLVideoElement;
    let stream: MediaStream;
    let sessionId: string | null;
  
    $: {
      const params = new URLSearchParams($querystring);
      sessionId = params.get("session");
      console.log("Session ID:", sessionId);
    }
  
    async function startVideoStream() {
      try {
        stream = await navigator.mediaDevices.getUserMedia({
          video: true,
          audio: false,
        });
  
        if (videoElement) {
          videoElement.srcObject = stream;
        }
  
        mediaRecorder = new MediaRecorder(stream, {
          mimeType: "video/webm;codecs=vp8",
          videoBitsPerSecond: 1000000, // 1 Mbps
        });
  
        mediaRecorder.ondataavailable = (event) => {
          if (event.data.size > 0 && ws?.readyState === WebSocket.OPEN) {
            try {
              ws.send(event.data); // Send binary data directly
              console.log(`Successfully sent video chunk: ${event.data.size} bytes`);
            } catch (error) {
              console.error('Failed to send video chunk:', error);
            }
          }
        };
  
        mediaRecorder.start(100); // Collect data every 100ms
      } catch (error) {
        console.error("Error starting video stream:", error);
      }
    }
  
    onMount(() => {
      if (sessionId) {
        // Connect to WebSocket server
        ws = new WebSocket("ws://localhost:8080/ws");
  
        ws.onopen = async () => {
          console.log("Connected to WebSocket server");
          await startVideoStream();
        };
  
        ws.onclose = () => {
          console.log("Disconnected from WebSocket server");
        };
  
        ws.onerror = (error) => {
          console.error("WebSocket error:", error);
        };
      }
    });
  
    onDestroy(() => {
      if (mediaRecorder?.state !== "inactive") {
        mediaRecorder?.stop();
      }
      if (ws) {
        ws.close();
      }
      if (stream) {
        stream.getTracks().forEach((track) => track.stop());
      }
    });
  </script>
  
  <div class="container mx-auto p-4">
    {#if sessionId}
      <div class="max-w-2xl mx-auto">
        <h1 class="text-2xl font-bold mb-4">Emergency Video Stream</h1>
        <p class="mb-4">Session ID: {sessionId}</p>
  
        <div class="aspect-video bg-gray-100 rounded-lg overflow-hidden">
          <video
            bind:this={videoElement}
            autoplay
            playsinline
            muted
            class="w-full h-full object-cover"
          ></video>
        </div>
  
        <p class="mt-4 text-sm text-gray-600">
          Your camera feed is being shared with emergency services.
        </p>
      </div>
    {:else}
      <p>Invalid session. Please use a valid streaming link.</p>
    {/if}
  </div>