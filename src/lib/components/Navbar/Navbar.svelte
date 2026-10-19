<script lang="ts">
    import "./Navbar.scss";

    import { sessionType, trackDetails } from "$lib/stores/sessionData";
    import { udpSession } from "$lib/stores/udp.svelte";

    let { connected } = $props();
    let sessionPacket = $derived(udpSession.data);
</script>

<div id="navbar">
    <div id="info">
        {#if connected}
            {#if sessionPacket !== null}
                <span id="connected">F1 Connected: {trackDetails[sessionPacket.track_id]?.name} - {sessionType[sessionPacket.session_type]}</span>
            {:else}
                <span id="connected">F1 Connected</span>
            {/if}
        {:else}
            <span>F1 Disconnected</span>
        {/if}
    </div>
</div>