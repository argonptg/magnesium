<script lang="ts">
  import Card from "../Card/Card.svelte";
  import "./CardRow.scss";
  import { udpCarStatus, udpLap, udpSession } from "$lib/stores/udp.svelte";
  import { actualTyre } from "$lib/stores/sessionData";
  import { formatDuration } from "$lib/utils";

  let laps: number[] = [];

  let cards = $derived.by(() => {
    const lapPacket = udpLap.data;
    const carPacket = udpCarStatus.data;
    const sessionPacket = udpSession.data;

    const carNum = lapPacket?.header.p_car_index ?? 0;

    const lapPlayer = lapPacket?.lap_data[carNum];
    const carPlayer = carPacket?.car_status_data[carNum];

    if (lapPlayer) {
      laps.push(lapPlayer?.last_lap_time_in_ms);
      laps.sort((a, b) => a - b); // arranjo decrescente
    }

    return [
      {
        title: "Lap",
        value: lapPlayer?.current_lap_num.toString() ?? "—",
      },
      {
        title: "Position",
        value: lapPlayer ? `P${lapPlayer.car_position}` : "—",
      },
      {
        title: "Tyre",
        value: actualTyre[carPlayer?.actual_tyre_compound ?? 0],
      },
      {
        title: "Last Lap Time",
        value: lapPlayer ? formatDuration(lapPlayer.last_lap_time_in_ms, "mm:ss.fff") : "—",
      },
      {
        title: "Fastest Lap",
        value: laps.length <= 0 ? "—" : formatDuration(laps[0], "mm:ss.fff"), // sempre o primeiro índex é a volta mais rápida
      },
      {
        title: "Remaining Time",
        value: sessionPacket ? formatDuration(sessionPacket.session_time_left * 1000, "mm:ss") : "—",
      },
    ];
  });
</script>

<div class="card-row">
  {#each cards as card}
    <Card title={card.title} data={card.value} />
  {/each}
</div>
