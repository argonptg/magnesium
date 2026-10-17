<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import "./Navbar.scss";
    import type { PacketSessionData } from "$lib/types/session";
    import { sessionType, trackDetails } from "$lib/sessionData";

    let { connected } = $props();

    let packetData = $state<PacketSessionData>();

    listen<PacketSessionData>("udp_session", (event) => {
        packetData = event.payload;
    });
</script>

<div id="navbar">
    <div id="info">
        {#if connected}
            {#if packetData !== undefined}
                <span>F1 Connected: {trackDetails[packetData.track_id]?.name} - {sessionType[packetData.session_type]}</span>
            {:else}
                <span>F1 Connected</span>
            {/if}
        {:else}
            <span>F1 Disconnected</span>
        {/if}
    </div>
</div>