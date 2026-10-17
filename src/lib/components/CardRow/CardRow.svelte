<script lang="ts">
    import Card from "../Card/Card.svelte";
    import "./CardRow.scss";
    import { udpLap } from "$lib/stores/udp.svelte";

    function formatDuration(totalMs: number) {
        const totalSeconds = Math.floor(totalMs / 1000);

        const minutes = String(Math.floor(totalSeconds / 60)).padStart(2, "0");
        const seconds = String(totalSeconds % 60).padStart(2, "0");
        const milliseconds = String(totalMs % 1000).padStart(3, "0");

        return `${minutes}:${seconds}.${milliseconds}`;
    }

    let laps: number[] = [];

    let cards = $derived.by(() => {
        const packet = udpLap.data;
        const player = packet?.lap_data[packet.header.p_car_index];

        if (player) {
            laps.push(player?.last_lap_time_in_ms);
            laps.sort((a, b) => a - b); // arranjo decrescente
        }

        return [
            {
                title: "Lap",
                value: player?.current_lap_num.toString() ?? "—",
            },
            {
                title: "Position",
                value: player ? `P${player.car_position}` : "—",
            },
            {
                title: "Tyre",
                value: "—",
            },
            {
                title: "Last Lap Time",
                value: player
                    ? formatDuration(player.last_lap_time_in_ms)
                    : "—",
            },
            {
                title: "Fastest Lap",
                value: formatDuration(laps[0]) // sempre o primeiro índex é a volta mais rápida
            },
            {
                title: "Remaining Time",
                value: "—",
            },
        ];
    });
</script>

<div class="card-row">
    {#each cards as card}
        <Card title={card.title} data={card.value} />
    {/each}
</div>
