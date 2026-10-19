<script lang="ts">
  import Shell from "$lib/components/Shell/Shell.svelte";
  import "$lib/styles/global.scss";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import CardRow from "$lib/components/CardRow/CardRow.svelte";
  import { registerUdpListeners } from "$lib/services/udpListener";
  import Leaderboard from "$lib/components/Leaderboard/Leaderboard.svelte";

  onMount(() => {
    let unlisten: (() => void) | undefined;
    let disposed = false;

    async function init() {
      try {
        const cleanup = await registerUdpListeners();

        if (disposed) {
          cleanup();
          return;
        }

        unlisten = cleanup;

        await invoke("try_connect", {
          ip: "0.0.0.0",
          port: 20777,
        });
      } catch (error) {
        console.error("Failed to initialize UDP:", error);
      }
    }

    void init();

    return () => {
      disposed = true;
      unlisten?.();
    };
  });
</script>

<main>
  <Shell>
    <div class="main-content">
      <CardRow />
      <Leaderboard />
    </div>
  </Shell>
</main>
