<script lang="ts">
  import { actualTyre } from "$lib/stores/sessionData";
    import { udpCarStatus, udpLap, udpParticipant } from "$lib/stores/udp.svelte";
  import { formatDuration } from "$lib/utils";
import "./Leaderboard.scss";

    let data = $derived.by(() => {
        const lapPacket = udpLap.data;
        const partPacket = udpParticipant.data;
        const statusPacket = udpCarStatus.data;

        if (!lapPacket || !partPacket || !statusPacket) return [];

        return Array.from(
            { length: partPacket.num_active_cars },
            (_, i) => {
                const l = lapPacket.lap_data[i];
                const p = partPacket.participants[i];
                const s = statusPacket.car_status_data[i];



                return {
                    id: i,
                    position: l.car_position,
                    number: p.race_number,
                    name: new TextDecoder().decode(
                        new Uint8Array(p.name).filter(b => b !== 0)
                    ),
                    interval:
                        formatDuration(
                            l.delta_to_car_in_front_minutes_part * 60000 +
                            l.delta_to_car_in_front_ms_part,
                            "ss.fff"
                        ),
                    leader:
                        formatDuration(
                            l.delta_to_race_leader_minutes_part * 60000 +
                            l.delta_to_race_leader_ms_part,
                            "ss.fff"
                        ),
                    lapTime: formatDuration(l.last_lap_time_in_ms, "mm:ss.fff"),
                    tyre: actualTyre[s.visual_tyre_compound]
                }
            }
        );
    });
</script>

<table>
    <thead>
        <tr>
            <th>Pos.</th>
            <th>No.</th>
            <th>Driver</th>
            <th>Interval</th>
            <th>Leader</th>
            <th>Lap Time</th>
            <th>Tyre</th>
        </tr>
    </thead>

    <tbody>
        {#each data as driver}
            <tr>
                <td>{driver.position}</td>
                <td>{driver.number}</td>
                <td>{driver.name}</td>
                <td>{driver.interval}</td>
                <td>{driver.leader}</td>
                <td>{driver.lapTime}</td>
                <td>{driver.tyre}</td>
            </tr>
        {/each}
    </tbody>
</table>