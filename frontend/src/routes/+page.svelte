<script lang="ts">
  import { onMount } from "svelte";

  let apiMessage: string = "Loading...";

  onMount(async () => {
    try {
      const res: Response = await fetch("/api/hello");
      if (!res.ok) throw new Error(`HTTP error ${res.status}`);
      apiMessage = await res.text();
    } catch (err) {
      if (err instanceof Error) {
        apiMessage = "Failed to load API: " + err.message;
      } else {
        apiMessage = "Unknown error";
      }
    }
  });
</script>

<main>
  <h1>Svelte + Rust API (TypeScript)</h1>
  <p>Message from API: {apiMessage}</p>
</main>