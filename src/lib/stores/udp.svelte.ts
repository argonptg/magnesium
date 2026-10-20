import type { PacketCarStatusData } from "$lib/types/carStatus";
import type { PacketLapData } from "$lib/types/lap";
import type { PacketParticipantsData } from "$lib/types/participants";
import type { PacketSessionData } from "$lib/types/session";
import type { PacketSessionHistoryData } from "$lib/types/sessionHistory";

// primeira vez q uso genericos
class PacketState<T> {
    data = $state.raw<T | null>(null);
}

export const udpLap = new PacketState<PacketLapData>();
export const udpParticipant = new PacketState<PacketParticipantsData>();
export const udpSession = new PacketState<PacketSessionData>();
export const udpCarStatus = new PacketState<PacketCarStatusData>();
export const udpSessionHistory = new PacketState<Record<number, PacketSessionHistoryData>>();

// não precisa ser packetstate por motivos bem obvios
export const udpConnection = $state({
    connected: false,
});
