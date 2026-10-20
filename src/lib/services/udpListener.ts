import { listen } from "@tauri-apps/api/event";

import {
    udpLap,
    udpParticipant,
    udpSession,
    udpConnection,
    udpCarStatus,
    udpSessionHistory
} from "$lib/stores/udp.svelte";

import type { PacketLapData } from "$lib/types/lap";
import type { PacketParticipantsData } from "$lib/types/participants";
import type { PacketSessionData } from "$lib/types/session";
import type { PacketCarStatusData } from "$lib/types/carStatus";
import type { PacketSessionHistoryData } from "$lib/types/sessionHistory";

export const registerUdpListeners = async () => {
    const unlisteners = await Promise.all([
        listen<PacketLapData>("udp_lap", ({ payload }) => {
            udpLap.data = payload;
        }),

        listen<PacketSessionData>("udp_session", ({ payload }) => {
            if (
                udpSession.data &&
                (
                    udpSession.data.header.session_uid !== payload.header.session_uid ||
                    udpSession.data.session_type !== payload.session_type
                )
            ) {
                udpSessionHistory.data = null;
            }
        
            udpSession.data = payload;
        }),

        listen<PacketParticipantsData>("udp_participants", ({ payload }) => {
            udpParticipant.data = payload
        }),

        listen<PacketCarStatusData>("udp_carstatus", ({ payload }) => {
            udpCarStatus.data = payload
        }),

        listen("udp_connected", () => {
            console.log("UDP CONNECTED EVENT RECEIVED");
            udpConnection.connected = true;
        }),

        listen<PacketSessionHistoryData>("udp_sessionhistory", ({ payload }) => {
            udpSessionHistory.data = {
                ...udpSessionHistory.data,
                [payload.car_idx]: payload
            };
        }),

        listen("udp_timeout", () => {
            udpConnection.connected = false;
        })
    ]);

    return () => unlisteners.forEach(unlisten => unlisten());
}