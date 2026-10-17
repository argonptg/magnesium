import type { TrackDetails } from "../types/session";

// adoro delegar tarefas a meus camaradas I.A
export const trackDetails: Partial<Record<number, TrackDetails>> = {
    0: {
        name: "Australian Grand Prix",
        track: "Albert Park",
        track_name: "Albert Park Circuit"
    },
    2: {
        name: "Chinese Grand Prix",
        track: "Shanghai",
        track_name: "Shanghai International Circuit"
    },
    3: {
        name: "Bahrain Grand Prix",
        track: "Sakhir",
        track_name: "Bahrain International Circuit"
    },
    4: {
        name: "Spanish Grand Prix",
        track: "Catalunya",
        track_name: "Circuit de Barcelona-Catalunya"
    },
    5: {
        name: "Monaco Grand Prix",
        track: "Monaco",
        track_name: "Circuit de Monaco"
    },
    6: {
        name: "Canadian Grand Prix",
        track: "Montreal",
        track_name: "Circuit Gilles Villeneuve"
    },
    7: {
        name: "British Grand Prix",
        track: "Silverstone",
        track_name: "Silverstone Circuit"
    },
    9: {
        name: "Hungarian Grand Prix",
        track: "Hungaroring",
        track_name: "Hungaroring"
    },
    10: {
        name: "Belgian Grand Prix",
        track: "Spa",
        track_name: "Circuit de Spa-Francorchamps"
    },
    11: {
        name: "Italian Grand Prix",
        track: "Monza",
        track_name: "Autodromo Nazionale Monza"
    },
    12: {
        name: "Singapore Grand Prix",
        track: "Marina Bay",
        track_name: "Marina Bay Street Circuit"
    },
    13: {
        name: "Japanese Grand Prix",
        track: "Suzuka",
        track_name: "Suzuka International Racing Course"
    },
    14: {
        name: "Abu Dhabi Grand Prix",
        track: "Yas Marina",
        track_name: "Yas Marina Circuit"
    },
    15: {
        name: "United States Grand Prix",
        track: "COTA",
        track_name: "Circuit of the Americas"
    },
    16: {
        name: "São Paulo Grand Prix",
        track: "Interlagos",
        track_name: "Autódromo José Carlos Pace"
    },
    17: {
        name: "Austrian Grand Prix",
        track: "Red Bull Ring",
        track_name: "Red Bull Ring"
    },
    19: {
        name: "Mexico City Grand Prix",
        track: "Hermanos Rodríguez",
        track_name: "Autódromo Hermanos Rodríguez"
    },
    20: {
        name: "Azerbaijan Grand Prix",
        track: "Baku",
        track_name: "Baku City Circuit"
    },
    26: {
        name: "Dutch Grand Prix",
        track: "Zandvoort",
        track_name: "Circuit Zandvoort"
    },
    27: {
        name: "Emilia Romagna Grand Prix",
        track: "Imola",
        track_name: "Autodromo Enzo e Dino Ferrari"
    },
    29: {
        name: "Saudi Arabian Grand Prix",
        track: "Jeddah",
        track_name: "Jeddah Corniche Circuit"
    },
    30: {
        name: "Miami Grand Prix",
        track: "Miami",
        track_name: "Miami International Autodrome"
    },
    31: {
        name: "Las Vegas Grand Prix",
        track: "Las Vegas",
        track_name: "Las Vegas Strip Circuit"
    },
    32: {
        name: "Qatar Grand Prix",
        track: "Lusail",
        track_name: "Lusail International Circuit"
    },
    39: {
        name: "British Grand Prix (Reverse)",
        track: "Silverstone (Reverse)",
        track_name: "Silverstone Circuit (Reverse)"
    },
    40: {
        name: "Austrian Grand Prix (Reverse)",
        track: "Red Bull Ring (Reverse)",
        track_name: "Red Bull Ring (Reverse)"
    },
    41: {
        name: "Dutch Grand Prix (Reverse)",
        track: "Zandvoort (Reverse)",
        track_name: "Circuit Zandvoort (Reverse)"
    }
};

export const sessionType: Record<number, string> = {
    0: "Unknown",
    1: "FP1",
    2: "FP2",
    3: "FP3",
    4: "ShP",
    5: "Q1",
    6: "Q2",
    7: "Q3",
    8: "ShQ",
    9: "OsQ",
    10: "SQ1",
    11: "SQ2",
    12: "SQ3",
    13: "ShSPQ",
    14: "OsSPQ",
    15: "Feature Race",
    16: "Feature Race 2",
    17: "Feature Race 3",
    18: "Time Trial",
};