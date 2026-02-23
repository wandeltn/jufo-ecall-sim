#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    German,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::German => "de",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::German => "Deutsch",
        }
    }
}

pub struct Translations {
    language: Language,
}

impl Translations {
    pub fn new(language: Language) -> Self {
        Translations { language }
    }

    pub fn t(&self, key: &str) -> String {
        match self.language {
            Language::English => self.en(key),
            Language::German => self.de(key),
        }
    }

    fn en(&self, key: &str) -> String {
        match key {
            // Navigation
            "nav.title" => "Emergency Response Simulator".to_string(),
            "nav.emergency_services" => "Emergency Services".to_string(),
            "nav.all_simulators" => "All Simulators".to_string(),
            "nav.auto_ecall" => "Auto e-call".to_string(),
            "nav.marine" => "Marine".to_string(),
            "nav.mobility" => "Mobility Assistance".to_string(),
            "nav.lift" => "Lift / Elevator".to_string(),
            "nav.about" => "About".to_string(),

            // Landing Page
            "landing.hero_title" => "Emergency Response Simulator".to_string(),
            "landing.hero_subtitle" => "A comprehensive training and simulation platform for emergency services, featuring realistic scenarios across multiple transportation and accessibility domains.".to_string(),
            "landing.launch_simulators" => "Launch Simulators".to_string(),
            "landing.learn_more" => "Learn More".to_string(),

            "landing.about_title" => "About This Project".to_string(),
            "landing.what_is_this" => "What is This?".to_string(),
            "landing.what_is_this_desc" => "This Emergency Response Simulator is a training platform designed to help emergency services personnel, first responders, and the public understand how to properly report emergencies. It provides realistic simulation scenarios that mimic actual emergency situations across different contexts: automotive emergencies (eCall system), maritime incidents, elevator emergencies, and accessibility-related emergencies.".to_string(),

            "landing.key_features" => "Key Features".to_string(),
            "landing.feature_presets" => "Realistic Emergency Presets: Quick-start scenarios based on actual incidents".to_string(),
            "landing.feature_geolocation" => "Automatic Geolocation: Real-time GPS integration with heading detection".to_string(),
            "landing.feature_prefill" => "Contact Information Prefill: Quick preset-based data entry".to_string(),
            "landing.feature_standards" => "European Standards: Compliant with eCall and emergency protocols".to_string(),

            "landing.simulators_title" => "Available Simulators".to_string(),
            "landing.auto_ecall_title" => "Auto e-Call".to_string(),
            "landing.auto_ecall_desc" => "Simulate automotive emergencies using the European eCall standard. Test vehicle emergency reporting with GPS tracking and realistic crash scenarios.".to_string(),
            "landing.marine_title" => "Marine Emergency".to_string(),
            "landing.marine_desc" => "Report maritime emergencies with crew information, vessel type, and water conditions. Realistic scenarios from sailing mishaps to cargo ship collisions.".to_string(),
            "landing.mobility_title" => "Mobility Assistance".to_string(),
            "landing.mobility_desc" => "Request assistance for wheelchair and mobility device emergencies. Report equipment failures, battery issues, and medical situations.".to_string(),
            "landing.lift_title" => "Lift / Elevator".to_string(),
            "landing.lift_desc" => "Handle elevator emergencies including mechanical failures, stuck doors, and medical emergencies within elevator cabins.".to_string(),

            "landing.how_it_works" => "How It Works".to_string(),
            "landing.step_1_select" => "Select Scenario".to_string(),
            "landing.step_1_desc" => "Choose from realistic preset emergency scenarios or enter custom details.".to_string(),
            "landing.step_2_enter" => "Enter Details".to_string(),
            "landing.step_2_desc" => "Fill in emergency information. GPS automatically detects your location.".to_string(),
            "landing.step_3_review" => "Review Data".to_string(),
            "landing.step_3_desc" => "Review all collected information including location and emergency details.".to_string(),
            "landing.step_4_submit" => "Submit".to_string(),
            "landing.step_4_desc" => "Submit the emergency report and see a summary of transmitted data.".to_string(),

            "landing.tech_title" => "Technology".to_string(),
            "landing.tech_framework" => "Modern Web Framework".to_string(),
            "landing.tech_framework_desc" => "Built with Leptos, a Rust web framework providing reactive, efficient user interfaces with minimal JavaScript overhead.".to_string(),
            "landing.tech_geolocation" => "Browser Geolocation".to_string(),
            "landing.tech_geolocation_desc" => "Integrates browser Geolocation API for accurate GPS positioning, heading detection, and real-time location updates with graceful fallback.".to_string(),
            "landing.tech_standards" => "Standards Compliant".to_string(),
            "landing.tech_standards_desc" => "Implements European standards including eCall protocol, realistic emergency data formats, and international emergency communication guidelines.".to_string(),

            "landing.cta_title" => "Ready to Get Started?".to_string(),
            "landing.cta_subtitle" => "Select a simulator and explore realistic emergency response scenarios.".to_string(),
            "landing.cta_button" => "Explore All Simulators".to_string(),

            // Simulators Page
            "simulators.title" => "Emergency Response Simulators".to_string(),
            "simulators.subtitle" => "Choose a simulator to train on emergency response procedures. Each scenario includes realistic presets and automatic location detection.".to_string(),
            
            "simulators.auto_ecall_title" => "Auto e-Call".to_string(),
            "simulators.auto_ecall_desc" => "Automotive emergency reporting system compliant with European eCall standards.".to_string(),
            "simulators.auto_ecall_feature1" => "Real-time GPS tracking".to_string(),
            "simulators.auto_ecall_feature2" => "Vehicle information".to_string(),
            "simulators.auto_ecall_feature3" => "Heading detection".to_string(),
            "simulators.auto_ecall_feature4" => "Position history".to_string(),
            
            "simulators.marine_title" => "Marine Emergency".to_string(),
            "simulators.marine_desc" => "Maritime incident reporting for boats, yachts, and commercial vessels.".to_string(),
            "simulators.marine_feature1" => "Vessel types".to_string(),
            "simulators.marine_feature2" => "Crew management".to_string(),
            "simulators.marine_feature3" => "Water conditions".to_string(),
            "simulators.marine_feature4" => "Captain info".to_string(),
            
            "simulators.mobility_title" => "Mobility Assistance".to_string(),
            "simulators.mobility_desc" => "Emergency assistance for wheelchair and mobility device users.".to_string(),
            "simulators.mobility_feature1" => "Equipment types".to_string(),
            "simulators.mobility_feature2" => "Failure types".to_string(),
            "simulators.mobility_feature3" => "Medical info".to_string(),
            "simulators.mobility_feature4" => "User details".to_string(),
            
            "simulators.lift_title" => "Lift/Elevator".to_string(),
            "simulators.lift_desc" => "Elevator and lift emergency response and rescue scenarios.".to_string(),
            "simulators.lift_feature1" => "Mechanical failures".to_string(),
            "simulators.lift_feature2" => "Stuck scenarios".to_string(),
            "simulators.lift_feature3" => "Medical support".to_string(),
            "simulators.lift_feature4" => "Building info".to_string(),
            
            "simulators.launch" => "Launch Simulator →".to_string(),
            
            "simulators.common_features" => "Common Features".to_string(),
            "simulators.feature_realistic" => "Realistic Presets:".to_string(),
            "simulators.feature_realistic_desc" => "Quick-start scenarios based on actual emergency incidents".to_string(),
            "simulators.feature_auto_geo" => "Automatic Geolocation:".to_string(),
            "simulators.feature_auto_geo_desc" => "Real-time GPS with heading and direction detection".to_string(),
            "simulators.feature_prefill" => "Quick Prefill:".to_string(),
            "simulators.feature_prefill_desc" => "Preset data instantly populates contact information".to_string(),
            "simulators.feature_manual" => "Manual Mode:".to_string(),
            "simulators.feature_manual_desc" => "Full control for custom scenarios".to_string(),
            "simulators.feature_summary" => "Data Summary:".to_string(),
            "simulators.feature_summary_desc" => "Review all information before submission".to_string(),
            
            "simulators.getting_started" => "Getting Started".to_string(),
            "simulators.step1_title" => "Choose a Simulator:".to_string(),
            "simulators.step1_desc" => "Select the type of emergency you want to simulate".to_string(),
            "simulators.step2_title" => "Select or Create:".to_string(),
            "simulators.step2_desc" => "Use a preset scenario or enter custom details".to_string(),
            "simulators.step3_title" => "Review Data:".to_string(),
            "simulators.step3_desc" => "Check all emergency information and location data".to_string(),
            "simulators.step4_title" => "Submit Report:".to_string(),
            "simulators.step4_desc" => "Complete the simulation and view the summary".to_string(),
            
            "simulators.back_home" => "← Back to Home".to_string(),

            // About Page
            "about.title" => "About Emergency Response Simulator".to_string(),
            "about.intro" => "This project is a modern, open-source training and simulation platform for emergency services, first responders, and the public. It provides realistic, interactive scenarios for reporting emergencies across automotive, maritime, elevator, and accessibility domains.".to_string(),
            
            "about.key_features" => "Key Features".to_string(),
            "about.feature1" => "Realistic Emergency Presets: Quick-start scenarios based on actual incidents".to_string(),
            "about.feature2" => "Automatic Geolocation: Real-time GPS integration with heading detection".to_string(),
            "about.feature3" => "Contact Information Prefill: Quick preset-based data entry".to_string(),
            "about.feature4" => "European Standards: Compliant with eCall and emergency protocols".to_string(),
            
            "about.how_it_works" => "How It Works".to_string(),
            "about.step1_title" => "Select Scenario:".to_string(),
            "about.step1_desc" => "Choose from realistic preset emergencies or enter custom details.".to_string(),
            "about.step2_title" => "Enter Details:".to_string(),
            "about.step2_desc" => "Fill in emergency information. GPS automatically detects your location.".to_string(),
            "about.step3_title" => "Review Data:".to_string(),
            "about.step3_desc" => "Check all collected information including location and emergency details.".to_string(),
            "about.step4_title" => "Submit:".to_string(),
            "about.step4_desc" => "Complete the simulation and see a summary of transmitted data.".to_string(),
            
            "about.technology" => "Technology".to_string(),
            "about.tech_leptos" => "Leptos Framework:".to_string(),
            "about.tech_leptos_desc" => "Rust-based, reactive web UI with minimal JavaScript overhead.".to_string(),
            "about.tech_geolocation" => "Browser Geolocation:".to_string(),
            "about.tech_geolocation_desc" => "Accurate GPS positioning, heading detection, and real-time updates.".to_string(),
            "about.tech_standards" => "European Standards:".to_string(),
            "about.tech_standards_desc" => "Implements eCall protocol and international emergency communication guidelines.".to_string(),
            
            "about.goals" => "Project Goals".to_string(),
            "about.goal1" => "Improve training for emergency services and public awareness.".to_string(),
            "about.goal2" => "Provide realistic, interactive simulations for education and testing.".to_string(),
            "about.goal3" => "Support accessibility and inclusivity in emergency reporting.".to_string(),
            
            "about.explore_btn" => "Explore Simulators".to_string(),

            // Emergency Simulators - Common UI
            "simulator.quick_selection" => "Quick Emergency Selection".to_string(),
            "simulator.quick_selection_desc" => "Choose a preset scenario that matches your situation. This is the fastest way to get help.".to_string(),
            "simulator.need_control" => "Need more control over the details?".to_string(),
            "simulator.manual_entry" => "✎ Enter Information Manually".to_string(),
            "simulator.back_presets" => "← Back to Presets".to_string(),
            "simulator.location_error" => "Location Error:".to_string(),
            "simulator.getting_location" => "Getting your location...".to_string(),
            "simulator.location_detected" => "Location detected!".to_string(),
            "simulator.enable_location" => "Enable Location Services".to_string(),
            "simulator.submit_report" => "Submit Report".to_string(),

            // eCall Simulator
            "ecall.title" => "Emergency Call System".to_string(),
            "ecall.subtitle" => "Enter all details relevant for starting an emergency call".to_string(),
            "ecall.activation_type" => "Activation Type:".to_string(),
            "ecall.vehicle_type" => "Vehicle Type:".to_string(),
            "ecall.propulsion_type" => "Propulsion Type:".to_string(),
            "ecall.vin_label" => "Vehicle Identification Number (VIN):".to_string(),
            "ecall.vin_placeholder" => "e.g., WVWZZZ3CZ9E123456".to_string(),
            "ecall.phone_label" => "Vehicle Phone Number:".to_string(),
            "ecall.phone_placeholder" => "e.g., +49123456789".to_string(),
            "ecall.restrained_occupants" => "Restrained Occupants:".to_string(),
            "ecall.driving_direction" => "Driving Direction:".to_string(),
            "ecall.current_location" => "Current Location".to_string(),
            "ecall.latitude" => "Latitude:".to_string(),
            "ecall.longitude" => "Longitude:".to_string(),
            "ecall.position_10s_ago" => "Position 10s Ago".to_string(),
            "ecall.position_20s_ago" => "Position 20s Ago".to_string(),

            // Boat Emergency Simulator
            "boat.title" => "Marine Emergency".to_string(),
            "boat.subtitle" => "Report maritime emergencies with vessel and crew information".to_string(),
            "boat.captain_name" => "Captain/Commander Name:".to_string(),
            "boat.captain_placeholder" => "e.g., John Smith".to_string(),
            "boat.phone" => "Contact Phone:".to_string(),
            "boat.boat_name" => "Vessel Name:".to_string(),
            "boat.boat_placeholder" => "e.g., Seaworthy".to_string(),
            "boat.vessel_type" => "Vessel Type:".to_string(),
            "boat.crew_size" => "Crew Size:".to_string(),
            "boat.emergency_type" => "Emergency Type:".to_string(),
            "boat.description" => "Emergency Description:".to_string(),
            "boat.description_placeholder" => "Describe what happened and any injuries...".to_string(),
            "boat.water_condition" => "Water Condition:".to_string(),

            // Lift/Elevator Emergency Simulator
            "lift.title" => "Lift/Elevator Emergency".to_string(),
            "lift.subtitle" => "Report elevator emergencies and request rescue assistance".to_string(),
            "lift.user_name" => "Occupant Name:".to_string(),
            "lift.user_placeholder" => "e.g., Sarah Johnson".to_string(),
            "lift.phone" => "Contact Phone:".to_string(),
            "lift.building_name" => "Building Name:".to_string(),
            "lift.building_placeholder" => "e.g., City Center Office Tower".to_string(),
            "lift.floor_level" => "Current Floor Level:".to_string(),
            "lift.floor_placeholder" => "e.g., 5".to_string(),
            "lift.lift_id" => "Lift/Elevator ID:".to_string(),
            "lift.lift_placeholder" => "e.g., A-03".to_string(),
            "lift.emergency_type" => "Emergency Type:".to_string(),
            "lift.description" => "Emergency Description:".to_string(),
            "lift.description_placeholder" => "Describe the situation and any medical needs...".to_string(),
            "lift.occupant_count" => "Number of Occupants:".to_string(),

            // Wheelchair/Mobility Emergency Simulator
            "wheelchair.title" => "Mobility Assistance Emergency".to_string(),
            "wheelchair.subtitle" => "Request assistance for wheelchair and mobility device emergencies".to_string(),
            "wheelchair.user_name" => "User Name:".to_string(),
            "wheelchair.user_placeholder" => "e.g., Maria Garcia".to_string(),
            "wheelchair.phone" => "Contact Phone:".to_string(),
            "wheelchair.device_type" => "Device Type:".to_string(),
            "wheelchair.failure_type" => "Failure Type:".to_string(),
            "wheelchair.medical_condition" => "Medical Condition:".to_string(),
            "wheelchair.description" => "Emergency Description:".to_string(),
            "wheelchair.description_placeholder" => "Describe the device failure or medical emergency...".to_string(),
            "wheelchair.location" => "Current Location:".to_string(),
            "wheelchair.requires_medical" => "Medical Assistance Needed:".to_string(),

            _ => key.to_string(),
        }
    }

    fn de(&self, key: &str) -> String {
        match key {
            // Navigation
            "nav.title" => "Rettungsdienst-Simulator".to_string(),
            "nav.emergency_services" => "Rettungsdienste".to_string(),
            "nav.all_simulators" => "Alle Simulatoren".to_string(),
            "nav.auto_ecall" => "Auto e-Call".to_string(),
            "nav.marine" => "Marine".to_string(),
            "nav.mobility" => "Mobilitätshilfe".to_string(),
            "nav.lift" => "Fahrstuhl / Aufzug".to_string(),
            "nav.about" => "Über".to_string(),

            // Landing Page
            "landing.hero_title" => "Rettungsdienst-Simulator".to_string(),
            "landing.hero_subtitle" => "Eine umfassende Trainings- und Simulationsplattform für Rettungsdienste mit realistischen Szenarien in mehreren Transport- und Mobilitätsbereichen.".to_string(),
            "landing.launch_simulators" => "Simulatoren starten".to_string(),
            "landing.learn_more" => "Mehr erfahren".to_string(),

            "landing.about_title" => "Über dieses Projekt".to_string(),
            "landing.what_is_this" => "Was ist das?".to_string(),
            "landing.what_is_this_desc" => "Der Rettungsdienst-Simulator ist eine Trainingsplattform, die Rettungskräften, Notfallhelfern und der Öffentlichkeit hilft, wie man Notfälle richtig meldet. Er bietet realistische Simulationsszenarien, die echte Notfallsituationen in verschiedenen Kontexten nachahmen: Fahzeugunfälle (eCall-System), Seenöte, Fahrstuhlemergenzien und Behindertennotzuständen.".to_string(),

            "landing.key_features" => "Hauptfunktionen".to_string(),
            "landing.feature_presets" => "Realistische Notfall-Voreinstellungen: Schnellstart-Szenarien basierend auf echten Vorfällen".to_string(),
            "landing.feature_geolocation" => "Automatische Geolokalisierung: GPS-Integration in Echtzeit mit Richtungserkennung".to_string(),
            "landing.feature_prefill" => "Kontaktinformation ausfüllen: Schnelle voreingestellte Dateneingabe".to_string(),
            "landing.feature_standards" => "Europäische Standards: Konform mit eCall und Notfallprotokollen".to_string(),

            "landing.simulators_title" => "Verfügbare Simulatoren".to_string(),
            "landing.auto_ecall_title" => "Auto e-Call".to_string(),
            "landing.auto_ecall_desc" => "Simulieren Sie Fahzeugunfälle mit dem europäischen eCall-Standard. Testen Sie Fahzeugnot-Berichte mit GPS-Verfolgung und realistischen Unfallszenarien.".to_string(),
            "landing.marine_title" => "Seenot".to_string(),
            "landing.marine_desc" => "Melden Sie Seenöte mit Besatzungsinformationen, Schiffstyp und Wasserbedingungen. Realistische Szenarien von Segeolunfällen bis zu Frachterschiffer-Kollisionen.".to_string(),
            "landing.mobility_title" => "Mobilitätshilfe".to_string(),
            "landing.mobility_desc" => "Fordern Sie Hilfe bei Notfällen mit Rollstühlen und Mobilitätsgeräten an. Melden Sie Ausrüstungsfehler, Batteriefehler und medizinische Situationen.".to_string(),
            "landing.lift_title" => "Fahrstuhl / Aufzug".to_string(),
            "landing.lift_desc" => "Handhaben Sie Fahrstuhlemergenzien einschließlich mechanischer Fehler, stecken bleibender Türen und medizinischer Emergenzien in Fahrstuhkabinen.".to_string(),

            "landing.how_it_works" => "Wie es funktioniert".to_string(),
            "landing.step_1_select" => "Szenario auswählen".to_string(),
            "landing.step_1_desc" => "Wählen Sie aus realistischen Notfall-Szenarien oder geben Sie benutzerdefinierte Details ein.".to_string(),
            "landing.step_2_enter" => "Details eingeben".to_string(),
            "landing.step_2_desc" => "Geben Sie Notfallinformationen ein. GPS erkennt Ihren Standort automatisch.".to_string(),
            "landing.step_3_review" => "Daten überprüfen".to_string(),
            "landing.step_3_desc" => "Überprüfen Sie alle gesammelten Informationen einschließlich Standort und Notfalldetails.".to_string(),
            "landing.step_4_submit" => "Absenden".to_string(),
            "landing.step_4_desc" => "Senden Sie den Notfallbericht ab und sehen Sie eine Zusammenfassung der übertragenen Daten.".to_string(),

            "landing.tech_title" => "Technologie".to_string(),
            "landing.tech_framework" => "Modernes Web-Framework".to_string(),
            "landing.tech_framework_desc" => "Gebaut mit Leptos, einem Rust-Web-Framework, das reaktive, effiziente Benutzeroberflächen mit minimalem JavaScript-Overhead bietet.".to_string(),
            "landing.tech_geolocation" => "Browser-Geolokalisierung".to_string(),
            "landing.tech_geolocation_desc" => "Integriert die Browser-Geolokalisierungs-API für genaue GPS-Positionierung, Richtungserkennung und Echtzeit-Standortupdates mit eleganter Fallback-Lösung.".to_string(),
            "landing.tech_standards" => "Normgerecht".to_string(),
            "landing.tech_standards_desc" => "Implementiert europäische Standards einschließlich eCall-Protokoll, realistische Notfall-Datenformate und internationale Richtlinien zur Notfallkommunikation.".to_string(),

            "landing.cta_title" => "Bereit zum Einstieg?".to_string(),
            "landing.cta_subtitle" => "Wählen Sie einen Simulator und erkunden Sie realistische Notfall-Szenarien.".to_string(),
            "landing.cta_button" => "Alle Simulatoren erkunden".to_string(),

            // Simulators Page
            "simulators.title" => "Rettungsdienst-Simulatoren".to_string(),
            "simulators.subtitle" => "Wählen Sie einen Simulator, um Verfahren für die Notfallreaktion zu trainieren. Jedes Szenario enthält realistische Voreinstellungen und automatische Standorterkennung.".to_string(),
            
            "simulators.auto_ecall_title" => "Auto e-Call".to_string(),
            "simulators.auto_ecall_desc" => "Fahzeugnot-Meldungssystem konform mit europäischem eCall-Standard.".to_string(),
            "simulators.auto_ecall_feature1" => "GPS-Verfolgung in Echtzeit".to_string(),
            "simulators.auto_ecall_feature2" => "Fahzeuginformationen".to_string(),
            "simulators.auto_ecall_feature3" => "Richtungserkennung".to_string(),
            "simulators.auto_ecall_feature4" => "Positionsverlauf".to_string(),
            
            "simulators.marine_title" => "Seenot".to_string(),
            "simulators.marine_desc" => "Seenot-Meldung für Boote, Yachten und Handelsschiffe.".to_string(),
            "simulators.marine_feature1" => "Schiffstypen".to_string(),
            "simulators.marine_feature2" => "Besatzungsverwaltung".to_string(),
            "simulators.marine_feature3" => "Wasserbedingungen".to_string(),
            "simulators.marine_feature4" => "Kapitäninformationen".to_string(),
            
            "simulators.mobility_title" => "Mobilitätshilfe".to_string(),
            "simulators.mobility_desc" => "Notfallhilfe für Rollstuhl- und Mobilitätsgerätebenutzer.".to_string(),
            "simulators.mobility_feature1" => "Ausrüstungstypen".to_string(),
            "simulators.mobility_feature2" => "Fehlertypen".to_string(),
            "simulators.mobility_feature3" => "Medizinische Informationen".to_string(),
            "simulators.mobility_feature4" => "Benutzerdetails".to_string(),
            
            "simulators.lift_title" => "Fahrstuhl/Aufzug".to_string(),
            "simulators.lift_desc" => "Fahrstuhl-Notfall-Reaktion und Rettungsszenarien.".to_string(),
            "simulators.lift_feature1" => "Mechanische Fehler".to_string(),
            "simulators.lift_feature2" => "Steckenbleib-Szenarien".to_string(),
            "simulators.lift_feature3" => "Medizinische Unterstützung".to_string(),
            "simulators.lift_feature4" => "Gebäudeinformationen".to_string(),
            
            "simulators.launch" => "Simulator starten →".to_string(),
            
            "simulators.common_features" => "Gemeinsame Funktionen".to_string(),
            "simulators.feature_realistic" => "Realistische Voreinstellungen:".to_string(),
            "simulators.feature_realistic_desc" => "Schnellstart-Szenarien basierend auf echten Notfällen".to_string(),
            "simulators.feature_auto_geo" => "Automatische Geolokalisierung:".to_string(),
            "simulators.feature_auto_geo_desc" => "GPS in Echtzeit mit Richtungs- und Lagerichtungerkennung".to_string(),
            "simulators.feature_prefill" => "Schnelles Ausfüllen:".to_string(),
            "simulators.feature_prefill_desc" => "Voreingestellte Daten füllen sofort Kontaktinformationen aus".to_string(),
            "simulators.feature_manual" => "Manueller Modus:".to_string(),
            "simulators.feature_manual_desc" => "Vollständige Kontrolle für benutzerdefinierte Szenarien".to_string(),
            "simulators.feature_summary" => "Datenzusammenfassung:".to_string(),
            "simulators.feature_summary_desc" => "Überprüfen Sie alle Informationen vor der Einreichung".to_string(),
            
            "simulators.getting_started" => "Erste Schritte".to_string(),
            "simulators.step1_title" => "Wählen Sie einen Simulator:".to_string(),
            "simulators.step1_desc" => "Wählen Sie die Art des Notfalls, den Sie simulieren möchten".to_string(),
            "simulators.step2_title" => "Wählen oder erstellen Sie:".to_string(),
            "simulators.step2_desc" => "Verwenden Sie ein voreingestelltes Szenario oder geben Sie benutzerdefinierte Details ein".to_string(),
            "simulators.step3_title" => "Daten überprüfen:".to_string(),
            "simulators.step3_desc" => "Überprüfen Sie alle Notfallinformationen und Standortdaten".to_string(),
            "simulators.step4_title" => "Bericht einreichen:".to_string(),
            "simulators.step4_desc" => "Schließen Sie die Simulation ab und sehen Sie die Zusammenfassung".to_string(),
            
            "simulators.back_home" => "← Zurück zur Startseite".to_string(),

            // About Page
            "about.title" => "Über den Rettungsdienst-Simulator".to_string(),
            "about.intro" => "Dieses Projekt ist eine moderne, quelloffene Trainings- und Simulationsplattform für Rettungsdienste, Notfallhelfer und die Öffentlichkeit. Es bietet realistische, interaktive Szenarien zum Melden von Notfällen in den Bereichen Fahzeuge, Seenot, Fahrstuhl und Barrierefreiheit.".to_string(),
            
            "about.key_features" => "Hauptmerkmale".to_string(),
            "about.feature1" => "Realistische Notfall-Voreinstellungen: Schnellstart-Szenarien basierend auf echten Vorfällen".to_string(),
            "about.feature2" => "Automatische Geolokalisierung: GPS-Integration in Echtzeit mit Richtungserkennung".to_string(),
            "about.feature3" => "Kontaktinformation ausfüllen: Schnelle voreingestellte Dateneingabe".to_string(),
            "about.feature4" => "Europäische Standards: Konform mit eCall und Notfallprotokollen".to_string(),
            
            "about.how_it_works" => "Wie es funktioniert".to_string(),
            "about.step1_title" => "Szenario auswählen:".to_string(),
            "about.step1_desc" => "Wählen Sie aus realistischen Notfall-Szenarien oder geben Sie benutzerdefinierte Details ein.".to_string(),
            "about.step2_title" => "Details eingeben:".to_string(),
            "about.step2_desc" => "Geben Sie Notfallinformationen ein. GPS erkennt Ihren Standort automatisch.".to_string(),
            "about.step3_title" => "Daten überprüfen:".to_string(),
            "about.step3_desc" => "Überprüfen Sie alle gesammelten Informationen einschließlich Standort und Notfalldetails.".to_string(),
            "about.step4_title" => "Absenden:".to_string(),
            "about.step4_desc" => "Schließen Sie die Simulation ab und sehen Sie eine Zusammenfassung der übertragenen Daten.".to_string(),
            
            "about.technology" => "Technologie".to_string(),
            "about.tech_leptos" => "Leptos-Framework:".to_string(),
            "about.tech_leptos_desc" => "Rust-basiert, reaktive Web-Benutzeroberfläche mit minimalem JavaScript-Overhead.".to_string(),
            "about.tech_geolocation" => "Browser-Geolokalisierung:".to_string(),
            "about.tech_geolocation_desc" => "Genaue GPS-Positionierung, Richtungserkennung und Echtzeit-Updates.".to_string(),
            "about.tech_standards" => "Europäische Standards:".to_string(),
            "about.tech_standards_desc" => "Implementiert eCall-Protokoll und internationale Richtlinien zur Notfallkommunikation.".to_string(),
            
            "about.goals" => "Projektziele".to_string(),
            "about.goal1" => "Verbesserung des Trainings für Rettungsdienste und Öffentlichkeitsbewusstsein.".to_string(),
            "about.goal2" => "Bereitstellung realistischer, interaktiver Simulationen zur Bildung und zum Testen.".to_string(),
            "about.goal3" => "Unterstützung von Barrierefreiheit und Inklusion bei der Notfallmeldung.".to_string(),
            
            "about.explore_btn" => "Simulatoren erkunden".to_string(),

            // Emergency Simulators - Common UI
            "simulator.quick_selection" => "Schnellauswahlbereich Notfall".to_string(),
            "simulator.quick_selection_desc" => "Wählen Sie ein Szenario aus, das zu Ihrer Situation passt. Dies ist der schnellste Weg, Hilfe zu erhalten.".to_string(),
            "simulator.need_control" => "Brauchen Sie mehr Kontrolle über die Details?".to_string(),
            "simulator.manual_entry" => "✎ Informationen manuell eingeben".to_string(),
            "simulator.back_presets" => "← Zurück zu Voreinstellungen".to_string(),
            "simulator.location_error" => "Standortfehler:".to_string(),
            "simulator.getting_location" => "Ihren Standort abrufen...".to_string(),
            "simulator.location_detected" => "Standort erkannt!".to_string(),
            "simulator.enable_location" => "Standortdienste aktivieren".to_string(),
            "simulator.submit_report" => "Bericht einreichen".to_string(),

            // eCall Simulator
            "ecall.title" => "Notrufsystem".to_string(),
            "ecall.subtitle" => "Geben Sie alle Informationen für einen Notruf ein".to_string(),
            "ecall.activation_type" => "Aktivierungstyp:".to_string(),
            "ecall.vehicle_type" => "Fahrzeugtyp:".to_string(),
            "ecall.propulsion_type" => "Antriebstyp:".to_string(),
            "ecall.vin_label" => "Fahrzeugidentifikationsnummer (FIN):".to_string(),
            "ecall.vin_placeholder" => "z.B. WVWZZZ3CZ9E123456".to_string(),
            "ecall.phone_label" => "Fahzeugtelefonnummer:".to_string(),
            "ecall.phone_placeholder" => "z.B. +49123456789".to_string(),
            "ecall.restrained_occupants" => "Angeschnallte Insassen:".to_string(),
            "ecall.driving_direction" => "Fahrtrichtung:".to_string(),
            "ecall.current_location" => "Aktueller Standort".to_string(),
            "ecall.latitude" => "Breitengrad:".to_string(),
            "ecall.longitude" => "Längengrad:".to_string(),
            "ecall.position_10s_ago" => "Position vor 10 Sekunden".to_string(),
            "ecall.position_20s_ago" => "Position vor 20 Sekunden".to_string(),

            // Boat Emergency Simulator
            "boat.title" => "Seenot".to_string(),
            "boat.subtitle" => "Melden Sie Seenöte mit Schiffs- und Besatzungsinformationen".to_string(),
            "boat.captain_name" => "Name des Kapitäns/Kommandanten:".to_string(),
            "boat.captain_placeholder" => "z.B. Johann Schmidt".to_string(),
            "boat.phone" => "Kontakttelefon:".to_string(),
            "boat.boat_name" => "Name des Schiffes:".to_string(),
            "boat.boat_placeholder" => "z.B. Seefest".to_string(),
            "boat.vessel_type" => "Schiffstyp:".to_string(),
            "boat.crew_size" => "Besatzungsgröße:".to_string(),
            "boat.emergency_type" => "Notfalltyp:".to_string(),
            "boat.description" => "Notfallbeschreibung:".to_string(),
            "boat.description_placeholder" => "Beschreiben Sie, was passiert ist und ob es Verletzungen gibt...".to_string(),
            "boat.water_condition" => "Wasserbedingung:".to_string(),

            // Lift/Elevator Emergency Simulator
            "lift.title" => "Fahrstuhl-/Aufzugsnotfall".to_string(),
            "lift.subtitle" => "Melden Sie Fahrstuhlemergenzien und fordern Sie Rettungshilfe an".to_string(),
            "lift.user_name" => "Name des Insassen:".to_string(),
            "lift.user_placeholder" => "z.B. Sarah Müller".to_string(),
            "lift.phone" => "Kontakttelefon:".to_string(),
            "lift.building_name" => "Gebäudename:".to_string(),
            "lift.building_placeholder" => "z.B. Innenstadt-Büroturm".to_string(),
            "lift.floor_level" => "Aktuelle Etagenebene:".to_string(),
            "lift.floor_placeholder" => "z.B. 5".to_string(),
            "lift.lift_id" => "Fahrstuhl-/Aufzugs-ID:".to_string(),
            "lift.lift_placeholder" => "z.B. A-03".to_string(),
            "lift.emergency_type" => "Notfalltyp:".to_string(),
            "lift.description" => "Notfallbeschreibung:".to_string(),
            "lift.description_placeholder" => "Beschreiben Sie die Situation und medizinische Anforderungen...".to_string(),
            "lift.occupant_count" => "Anzahl der Insassen:".to_string(),

            // Wheelchair/Mobility Emergency Simulator
            "wheelchair.title" => "Notfall der Mobilitätshilfe".to_string(),
            "wheelchair.subtitle" => "Fordern Sie Hilfe bei Nothilfen für Rollstühle und Mobilitätsgeräte an".to_string(),
            "wheelchair.user_name" => "Benutzername:".to_string(),
            "wheelchair.user_placeholder" => "z.B. Maria García".to_string(),
            "wheelchair.phone" => "Kontakttelefon:".to_string(),
            "wheelchair.device_type" => "Gerätetyp:".to_string(),
            "wheelchair.failure_type" => "Fehlertyp:".to_string(),
            "wheelchair.medical_condition" => "Medizinische Bedingung:".to_string(),
            "wheelchair.description" => "Notfallbeschreibung:".to_string(),
            "wheelchair.description_placeholder" => "Beschreiben Sie den Geräteausfall oder den medizinischen Notfall...".to_string(),
            "wheelchair.location" => "Aktueller Standort:".to_string(),
            "wheelchair.requires_medical" => "Medizinische Hilfe erforderlich:".to_string(),

            _ => key.to_string(),
        }
    }
}
