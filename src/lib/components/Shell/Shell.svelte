<script lang="ts">
    import Navbar from "../Navbar/Navbar.svelte";
    import Sidebar from "../Sidebar/Sidebar.svelte";
    import { listen } from "@tauri-apps/api/event";

    import "./Shell.scss";

    let { children } = $props();
    let connected = $state(false);

    listen("udp_connected", () => {
        connected = true;
    })

    listen("udp_timeout", async () => {
        connected = false;
    })
</script>

<Sidebar/>

<div id="content">
    <Navbar connected={connected}/>
    {@render children()}
</div>
