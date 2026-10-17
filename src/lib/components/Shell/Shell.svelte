<script lang="ts">
    import Navbar from "../Navbar/Navbar.svelte";
    import Sidebar from "../Sidebar/Sidebar.svelte";
    import { listen } from "@tauri-apps/api/event";

    import "./Shell.scss";
    import { invoke } from "@tauri-apps/api/core";

    let { children } = $props();
    let connected = $state(false);

    listen("udp_connected", (event) => {
        connected = true;
    })

    listen("udp_timeout", async (event) => {
        connected = false;

        // try to connect again
        await invoke("try_connect", { ip: "0.0.0.0", port: 20777 });
    })
</script>

<Sidebar/>

<div id="content">
    <Navbar connected={connected}/>
    {@render children()}
</div>
