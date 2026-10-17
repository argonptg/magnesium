<script lang="ts">
    import Navbar from "../Navbar/Navbar.svelte";
    import Sidebar from "../Sidebar/Sidebar.svelte";

    import "./Shell.scss";
    import { udpConnection } from "$lib/stores/udp.svelte";

    let { children } = $props();
    let connected = $derived(udpConnection.connected);

    $effect(() => {
    console.log("Connection state:", udpConnection.connected);
});
</script>

<Sidebar/>

<div id="content">
    <Navbar connected={connected}/>
    {@render children()}
</div>
