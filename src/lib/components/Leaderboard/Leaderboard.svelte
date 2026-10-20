<script lang="ts">
    import { actualTyre, driverName } from "$lib/stores/sessionData";
    import {
        udpCarStatus,
        udpLap,
        udpParticipant,
        udpSession,
        udpSessionHistory
    } from "$lib/stores/udp.svelte";

    import type { PacketSessionHistoryData } from "$lib/types/sessionHistory";
    import { formatDuration } from "$lib/utils";
    import "./Leaderboard.scss";

    const decoder = new TextDecoder();

    type SessionMode = "race" | "practice" | "quali";

    let mode = $derived.by((): SessionMode => {
        const type = udpSession.data?.session_type;

        if (type !== undefined) {
            if (type >= 1 && type <= 4) return "practice";
            if (type >= 5 && type <= 14) return "quali";
            if (type >= 15 && type <= 17) return "race";
            if (type === 18) return "practice"; // Time Trial
        }

        return "race";
    });

    function decodeName(bytes: number[]): string {
        const end = bytes.indexOf(0);

        return decoder.decode(
            new Uint8Array(end === -1 ? bytes : bytes.slice(0, end))
        );
    }

    function formatGap(ms: number): string {
        const format = ms >= 80_000 ? "mm:ss.fff" : "ss.fff";

        return formatDuration(ms, format)
            .replace(/^0(?=\d[:.])/, "");
    }

    function raceDelta(minutes: number, milliseconds: number): string {
        // Valores sentinela usados para intervalos indisponíveis.
        if (minutes === 255 || milliseconds === 65535) return "—";

        const totalMs = minutes * 60_000 + milliseconds;

        if (totalMs <= 0) return "—";

        return `+${formatGap(totalMs)}`;
    }

    function getBestLap(
        history: PacketSessionHistoryData | undefined
    ): number | null {
        if (!history) return null;

        // O número da volta é 1-based.
        const index = history.best_lap_time_lap_num - 1;

        if (index < 0 || index >= history.num_laps) return null;

        const lap = history.lap_history_data[index];

        return lap && lap.lap_time_in_ms > 0
            ? lap.lap_time_in_ms
            : null;
    }

    let data = $derived.by(() => {
        const lapPacket = udpLap.data;
        const partPacket = udpParticipant.data;
        const statusPacket = udpCarStatus.data;
        const histories = udpSessionHistory.data;

        if (!lapPacket || !partPacket) return [];

        const count = Math.min(
            partPacket.num_active_cars,
            lapPacket.lap_data.length,
            partPacket.participants.length
        );

        const rows = Array.from({ length: count }, (_, i) => {
            const l = lapPacket.lap_data[i];
            const p = partPacket.participants[i];
            const s = statusPacket?.car_status_data[i];
            const history = histories?.[i];

            const name = decodeName(p.name);
            const bestLapMs = getBestLap(history);

            return {
                id: i,
                position: l.car_position,
                number: p.race_number,
                name: driverName[name] ?? name,

                interval: raceDelta(
                    l.delta_to_car_in_front_minutes_part,
                    l.delta_to_car_in_front_ms_part
                ),

                leader: raceDelta(
                    l.delta_to_race_leader_minutes_part,
                    l.delta_to_race_leader_ms_part
                ),

                lapTime: l.last_lap_time_in_ms > 0
                    ? formatDuration(l.last_lap_time_in_ms, "mm:ss.fff")
                    : "—",

                bestLapMs,

                laps: history?.num_laps
                    ?? Math.max(0, l.current_lap_num - 1),

                tyre: s
                    ? actualTyre[s.visual_tyre_compound] ?? "—"
                    : "—"
            };
        });

        if (mode === "race") {
            rows.sort((a, b) =>
                (a.position || 999) - (b.position || 999)
            );
        } else {
            rows.sort((a, b) =>
                (a.bestLapMs ?? Infinity) -
                (b.bestLapMs ?? Infinity)
                || a.id - b.id
            );
        }

        const leaderBestLap =
            rows.find(row => row.bestLapMs !== null)?.bestLapMs ?? null;

        return rows.map((driver, index) => ({
            ...driver,

            position: mode === "race"
                ? driver.position
                : driver.bestLapMs !== null
                    ? index + 1
                    : null,

            gap: mode === "race" ||
                 driver.bestLapMs === null ||
                 leaderBestLap === null ||
                 index === 0
                    ? "—"
                    : `+${formatGap(driver.bestLapMs - leaderBestLap)}`,

            bestLap: driver.bestLapMs !== null
                ? formatDuration(driver.bestLapMs, "mm:ss.fff")
                : "—"
        }));
    });
</script>

<table>
    <thead>
        <tr>
            <th>Pos.</th>
            <th>No.</th>
            <th>Driver</th>

            {#if mode === "race"}
                <th>Interval</th>
                <th>Leader</th>
                <th>Lap Time</th>
            {:else if mode === "practice"}
                <th>Gap</th>
                <th>Best Lap</th>
                <th>Laps</th>
            {:else}
                <th>Gap</th>
                <th>Best Lap</th>
            {/if}

            <th>Tyre</th>
        </tr>
    </thead>

    <tbody>
        {#each data as driver (driver.id)}
            <tr>
                <td>{driver.position ?? "—"}</td>
                <td>{driver.number}</td>
                <td>{driver.name}</td>

                {#if mode === "race"}
                    <td>{driver.interval}</td>
                    <td>{driver.leader}</td>
                    <td>{driver.lapTime}</td>
                {:else if mode === "practice"}
                    <td>{driver.gap}</td>
                    <td>{driver.bestLap}</td>
                    <td>{driver.laps}</td>
                {:else}
                    <td>{driver.gap}</td>
                    <td>{driver.bestLap}</td>
                {/if}

                <td>{driver.tyre}</td>
            </tr>
        {/each}
    </tbody>
</table>