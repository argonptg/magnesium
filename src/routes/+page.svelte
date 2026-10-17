<script lang="ts">
  import Card from '$lib/components/Card/Card.svelte';

  import Shell from "$lib/components/Shell/Shell.svelte"
  import "$lib/styles/global.scss"
  import type { PacketParticipantsData } from "$lib/types/participants";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
    import CardRow from '$lib/components/CardRow/CardRow.svelte';

  onMount(async () => {
    // try to connect
    await invoke("try_connect", { ip: "0.0.0.0", port: 20777 });
  });
</script>

<main>
  <Shell>
    <div class="main-content">      
      <CardRow/>
    </div>
  </Shell>
</main>
