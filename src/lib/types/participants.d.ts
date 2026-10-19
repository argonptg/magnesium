import { F1Header } from "./header"

interface ParticipantData {
    ai_controlled: number,                 
    driver_id: number,                     
    network_id: number,                    
    team_id: number,                       
    my_team: number,                       
    race_number: number,                   
    nationality: number,                   
    name: number[],                    
    your_telemetry: number,                 
    show_online_names: number,              
    tech_level: number,                   
    platform: number,                       
    num_colours: number,                    
    livery_colours: LiveryColor[],
}

interface LiveryColor {
    red: number,
    green: number,
    blue: number
}

export interface PacketParticipantsData {
    header: F1Header,
    num_active_cars: number,
    participants: ParticipantData[]
}

