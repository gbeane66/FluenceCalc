// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn fluence(power: &str, pulse_picker: &str, rep_rate: &str, spot_diameter: &str) -> String {
    if power == "" || pulse_picker == "" || rep_rate == "" || spot_diameter == "" {
        format!("")
    } else {
        let power_val: f64 = power.parse().unwrap(); // in mW
        let pulse_picker_val: f64 = pulse_picker.parse().unwrap();
        let rep_rate_val: f64 = rep_rate.parse().unwrap(); // in kHz
        let spot_diameter_val: f64 = spot_diameter.parse().unwrap(); // in mm
        let spot_area: f64 = 3.14159265 * (0.1*spot_diameter_val/2.0).powf(2.0); // area in cm2
        let pulse_energy: f64 = power_val * pulse_picker_val / (rep_rate_val); // pulse energy in uJ
        let fluence_val: f64 = pulse_energy / spot_area; // in uJ/cm2

        // format!("{:.2}",pulse_energy)
        format!("Fluence: {:.2} uJ/cm2", fluence_val)
    }
}
#[tauri::command]
fn energy(power: &str, pulse_picker: &str, rep_rate: &str, spot_diameter: &str) -> String {
    if power == "" || pulse_picker == "" || rep_rate == "" || spot_diameter == "" {
        format!("")
    } else {
        let power_val: f64 = power.parse().unwrap(); // in mW
        let pulse_picker_val: f64 = pulse_picker.parse().unwrap();
        let rep_rate_val: f64 = rep_rate.parse().unwrap(); // in kHz
        
        let pulse_energy: f64 = power_val * pulse_picker_val / (rep_rate_val); // pulse energy in uJ
        // format!("{:.2}",pulse_energy)
        format!("Energy: {:.2} uJ", pulse_energy)
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![energy,fluence])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
