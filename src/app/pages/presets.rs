use serde::{Deserialize, Serialize};

// Boat Emergency Presets
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoatEmergencyPreset {
    pub name: String,
    pub vessel_type: String,
    pub crew_size: String,
    pub emergency_type: String,
    pub water_condition: String,
    pub description: String,
    pub captain_name: String,
    pub phone: String,
    pub boat_name: String,
}

pub fn get_boat_presets() -> Vec<BoatEmergencyPreset> {
    vec![
        BoatEmergencyPreset {
            name: "Sinking Sailboat".to_string(),
            vessel_type: "Sailboat".to_string(),
            crew_size: "2".to_string(),
            emergency_type: "Collision".to_string(),
            water_condition: "Sinking".to_string(),
            description: "Sailboat is taking on water and sinking rapidly".to_string(),
            captain_name: "Captain Hans Schmidt".to_string(),
            phone: "+49-123-456789".to_string(),
            boat_name: "Lady of the North".to_string(),
        },
        BoatEmergencyPreset {
            name: "Motorboat Engine Failure".to_string(),
            vessel_type: "Motorboat".to_string(),
            crew_size: "3".to_string(),
            emergency_type: "Engine Failure".to_string(),
            water_condition: "Stable".to_string(),
            description: "Engine failure - vessel is drifting without power".to_string(),
            captain_name: "Captain Marcus Weber".to_string(),
            phone: "+49-234-567890".to_string(),
            boat_name: "Atlantic Explorer".to_string(),
        },
        BoatEmergencyPreset {
            name: "Yacht Fire".to_string(),
            vessel_type: "Yacht".to_string(),
            crew_size: "5".to_string(),
            emergency_type: "Fire".to_string(),
            water_condition: "Stable".to_string(),
            description: "Fire outbreak in engine compartment - crew evacuating".to_string(),
            captain_name: "Captain Anna Mueller".to_string(),
            phone: "+49-345-678901".to_string(),
            boat_name: "Luxury Dreams".to_string(),
        },
        BoatEmergencyPreset {
            name: "Fishing Boat Medical Emergency".to_string(),
            vessel_type: "Fishing Boat".to_string(),
            crew_size: "4".to_string(),
            emergency_type: "Medical".to_string(),
            water_condition: "Stable".to_string(),
            description:
                "Crew member has suffered a serious injury - immediate medical attention needed"
                    .to_string(),
            captain_name: "Captain Klaus Bergman".to_string(),
            phone: "+49-456-789012".to_string(),
            boat_name: "Steady Catch".to_string(),
        },
        BoatEmergencyPreset {
            name: "Cargo Ship Collision".to_string(),
            vessel_type: "Cargo Ship".to_string(),
            crew_size: "12".to_string(),
            emergency_type: "Collision".to_string(),
            water_condition: "Taking Water".to_string(),
            description: "Collision with another vessel - hull breach in cargo hold".to_string(),
            captain_name: "Captain Erik Olsen".to_string(),
            phone: "+49-567-890123".to_string(),
            boat_name: "Trade Winds".to_string(),
        },
    ]
}

// Lift Emergency Presets
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiftEmergencyPreset {
    pub name: String,
    pub current_floor: String,
    pub people_trapped: String,
    pub emergency_type: String,
    pub description: String,
    pub medical_info: String,
    pub user_name: String,
    pub phone: String,
    pub building_name: String,
}

pub fn get_lift_presets() -> Vec<LiftEmergencyPreset> {
    vec![
        LiftEmergencyPreset {
            name: "Lift Stuck Between Floors".to_string(),
            current_floor: "3".to_string(),
            people_trapped: "4".to_string(),
            emergency_type: "Stuck".to_string(),
            description: "Lift stopped between floors - no movement for 10 minutes".to_string(),
            medical_info: String::new(),
            user_name: "Johann Weber".to_string(),
            phone: "+49-123-456789".to_string(),
            building_name: "Central Tower Office Building".to_string(),
        },
        LiftEmergencyPreset {
            name: "Door Stuck - Single Person".to_string(),
            current_floor: "2".to_string(),
            people_trapped: "1".to_string(),
            emergency_type: "Door Stuck".to_string(),
            description: "Lift doors will not open - person inside is panicking".to_string(),
            medical_info: String::new(),
            user_name: "Maria Fischer".to_string(),
            phone: "+49-234-567890".to_string(),
            building_name: "Stadtmitte Shopping Complex".to_string(),
        },
        LiftEmergencyPreset {
            name: "Medical Emergency in Lift".to_string(),
            current_floor: "5".to_string(),
            people_trapped: "2".to_string(),
            emergency_type: "Medical Emergency".to_string(),
            description: "Person in lift has collapsed - consciousness unknown".to_string(),
            medical_info: "Patient appears unresponsive - breathing shallow".to_string(),
            user_name: "Klaus Hoffman".to_string(),
            phone: "+49-345-678901".to_string(),
            building_name: "Medical Center Hospital".to_string(),
        },
        LiftEmergencyPreset {
            name: "Mechanical Failure".to_string(),
            current_floor: "G".to_string(),
            people_trapped: "3".to_string(),
            emergency_type: "Mechanical Failure".to_string(),
            description: "Lift mechanical system failure - emergency stop activated".to_string(),
            medical_info: String::new(),
            user_name: "Petra Mueller".to_string(),
            phone: "+49-456-789012".to_string(),
            building_name: "Central Train Station".to_string(),
        },
        LiftEmergencyPreset {
            name: "Emergency Stop Activated".to_string(),
            current_floor: "B1".to_string(),
            people_trapped: "6".to_string(),
            emergency_type: "Emergency Stop".to_string(),
            description: "Emergency stop button activated - immediate assistance needed"
                .to_string(),
            medical_info: String::new(),
            user_name: "Robert Schneider".to_string(),
            phone: "+49-567-890123".to_string(),
            building_name: "Hauptplatz Parking Building".to_string(),
        },
    ]
}

// Wheelchair Emergency Presets
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WheelchairEmergencyPreset {
    pub name: String,
    pub wheelchair_model: String,
    pub emergency_type: String,
    pub description: String,
    pub medical_info: String,
    pub user_name: String,
    pub phone: String,
    pub location: String,
}

pub fn get_wheelchair_presets() -> Vec<WheelchairEmergencyPreset> {
    vec![
        WheelchairEmergencyPreset {
            name: "Power Wheelchair Battery Dead".to_string(),
            wheelchair_model: "Power Wheelchair".to_string(),
            emergency_type: "Battery Dead".to_string(),
            description: "Wheelchair battery has completely discharged - unable to move".to_string(),
            medical_info: String::new(),
            user_name: "Anna Richter".to_string(),
            phone: "+49-123-456789".to_string(),
            location: "Marienplatz Square, Munich".to_string(),
        },
        WheelchairEmergencyPreset {
            name: "Manual Wheelchair Breakdown".to_string(),
            wheelchair_model: "Manual Wheelchair".to_string(),
            emergency_type: "Breakdown".to_string(),
            description: "Wheelchair wheel damaged or mechanical failure - unable to self-propel".to_string(),
            medical_info: String::new(),
            user_name: "Thomas Brecht".to_string(),
            phone: "+49-234-567890".to_string(),
            location: "Main Railway Station, Berlin".to_string(),
        },
        WheelchairEmergencyPreset {
            name: "Scooter Flat Tire".to_string(),
            wheelchair_model: "Scooter".to_string(),
            emergency_type: "Flat Tire".to_string(),
            description: "Scooter has flat tire - vehicle immobilized".to_string(),
            medical_info: String::new(),
            user_name: "Greta Hansen".to_string(),
            phone: "+49-345-678901".to_string(),
            location: "Altstadt District, Dresden".to_string(),
        },
        WheelchairEmergencyPreset {
            name: "Mobility Assistance Needed".to_string(),
            wheelchair_model: "Power Wheelchair".to_string(),
            emergency_type: "Mobility Assistance".to_string(),
            description: "Need physical assistance to move - wheelchair functional but user unable to operate".to_string(),
            medical_info: String::new(),
            user_name: "Hans Neumann".to_string(),
            phone: "+49-456-789012".to_string(),
            location: "Town Center, Hamburg".to_string(),
        },
        WheelchairEmergencyPreset {
            name: "Motorized Scooter Medical Emergency".to_string(),
            wheelchair_model: "Motorized Scooter".to_string(),
            emergency_type: "Medical Emergency".to_string(),
            description: "Medical emergency while operating scooter - immediate assistance required".to_string(),
            medical_info: "User experiencing chest pain - vitals unstable".to_string(),
            user_name: "Ingrid Braun".to_string(),
            phone: "+49-567-890123".to_string(),
            location: "Kunsthalle Park, Frankfurt".to_string(),
        },
    ]
}

// E-call Emergency Presets
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct EcallEmergencyPreset {
    pub name: String,
    pub activation_type: String,
    pub vehicle_type: String,
    pub propulsion_type: String,
    pub restrained_occupants: String,
    pub driving_direction: String,
    pub description: String,
    pub vin: String,
    pub vehicle_phone: String,
}

pub fn get_ecall_presets() -> Vec<EcallEmergencyPreset> {
    vec![
        EcallEmergencyPreset {
            name: "Frontal Collision - Multiple Vehicles".to_string(),
            activation_type: "Accident".to_string(),
            vehicle_type: "PKW".to_string(),
            propulsion_type: "Diesel".to_string(),
            restrained_occupants: "2".to_string(),
            driving_direction: "North".to_string(),
            description: "Head-on collision between two cars at highway intersection".to_string(),
            vin: "WBADT43452G216401".to_string(),
            vehicle_phone: "+49-123-456789".to_string(),
        },
        EcallEmergencyPreset {
            name: "Vehicle Rollover".to_string(),
            activation_type: "Accident".to_string(),
            vehicle_type: "PKW".to_string(),
            propulsion_type: "Benzin".to_string(),
            restrained_occupants: "3".to_string(),
            driving_direction: "East".to_string(),
            description:
                "Vehicle rolled over on curve - doors locked, passengers need immediate help"
                    .to_string(),
            vin: "WVWZZZ3CZ9E123456".to_string(),
            vehicle_phone: "+49-987-654321".to_string(),
        },
        EcallEmergencyPreset {
            name: "SOS Button Activation".to_string(),
            activation_type: "SOS Button".to_string(),
            vehicle_type: "LKW".to_string(),
            propulsion_type: "Diesel".to_string(),
            restrained_occupants: "1".to_string(),
            driving_direction: "South".to_string(),
            description: "Driver pressed SOS button - medical emergency in cabin".to_string(),
            vin: "WKLWZZZ2Z9C123789".to_string(),
            vehicle_phone: "+49-555-111222".to_string(),
        },
        EcallEmergencyPreset {
            name: "Electric Vehicle Fire".to_string(),
            activation_type: "Accident".to_string(),
            vehicle_type: "PKW".to_string(),
            propulsion_type: "Elektro".to_string(),
            restrained_occupants: "2".to_string(),
            driving_direction: "West".to_string(),
            description: "Battery fire in electric vehicle after minor collision".to_string(),
            vin: "TMAAA4X20N6012345".to_string(),
            vehicle_phone: "+49-666-333444".to_string(),
        },
        EcallEmergencyPreset {
            name: "Motorcycle Collision".to_string(),
            activation_type: "Accident".to_string(),
            vehicle_type: "Krad".to_string(),
            propulsion_type: "Benzin".to_string(),
            restrained_occupants: "1".to_string(),
            driving_direction: "North".to_string(),
            description: "Motorcycle hit by car - rider unconscious on roadside".to_string(),
            vin: "YMMA8K5A940000001".to_string(),
            vehicle_phone: "+49-777-888999".to_string(),
        },
    ]
}
