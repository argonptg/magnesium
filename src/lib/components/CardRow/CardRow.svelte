<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import Card from "../Card/Card.svelte";
    import "./CardRow.scss";
    import type { PacketLapData } from "$lib/types/lap";

    let cards = $state([
        { title: "Lap", value: "—" },
        { title: "Position", value: "—" },
        { title: "Tyre", value: "—" },
        { title: "Current Lap Time", value: "—" },
        { title: "Remaining Time", value: "—" }
    ]);

    function formatDuration(totalMs: number) {
        const totalSeconds = Math.floor(totalMs / 1000);
  
        const minutes = String(Math.floor(totalSeconds / 60)).padStart(2, '0');
        const seconds = String(totalSeconds % 60).padStart(2, '0');
        const milliseconds = String(totalMs % 1000).padStart(3, '0');

        return `${minutes}:${seconds}.${milliseconds}`;
    }

    listen<PacketLapData>("udp_lap", (event) => {
        let packetData = event.payload;
        let carNum = packetData.header.p_car_index;

        // com o udp_lap dá pra pegar
        // - a volta atual;
        // - a posição atual;
        // - o tempo da volta atual

        cards[0].value = packetData.lap_data[carNum].current_lap_num.toString();
        cards[1].value = `P${packetData.lap_data[carNum].car_position}`;
        cards[3].value = formatDuration(packetData.lap_data[carNum].last_lap_time_in_ms);
    });


</script>

<div class="card-row">
    {#each cards as card}
        <Card title={card.title} data={card.value} />
    {/each}
</div>
