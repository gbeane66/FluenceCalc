import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [energyMsg, setEnergyMsg] = useState("");
  const [fluenceMsg, setFluenceMsg] = useState("");
  const [power, setPower] = useState("");
  const [pulsePicker, setPulsePicker] = useState("");
  const [repRate, setRepRate] = useState("");
  const [spotDiameter, setSpotDiameter] = useState("");

  async function energy() {
    setEnergyMsg(await invoke("energy", { power, pulsePicker, repRate, spotDiameter })); //this is the part from the the rust main.rs
  }

  async function fluence() {
        setFluenceMsg(await invoke("fluence", { power, pulsePicker, repRate, spotDiameter })); //this is the part from the the rust main.rs
  }

  return (
    <div className="container">

      <form
        className="column"
        onSubmit={(e) => {
          e.preventDefault();
          energy();
          fluence();
        }}
      >
        <label>Power (mW)</label>
        <input
          id="power-input"
          onChange={(e) => setPower(e.currentTarget.value)}
          placeholder="Power (mW)"
        />
        <label>Pulse Picker</label>
        <input
          id="pulsePicker-input"
          onChange={(e) => setPulsePicker(e.currentTarget.value)}
          placeholder="Pulse picker"
        />
        <label>Rep Rate (kHz)</label>
        <input
          id="repRate-input"
          onChange={(e) => setRepRate(e.currentTarget.value)}
          placeholder="Repetition Rate (kHz)"
        />
        <label>Spot diameter (mm)</label>
        <input
          id="spotDiameter-input"
          onChange={(e) => setSpotDiameter(e.currentTarget.value)}
          placeholder="Spot diameter (mm)"
        />
        <br></br>
        <button type="submit">Calculate</button>
        <p id="energy-val">{energyMsg}</p>
        <p id="fluence-val">{fluenceMsg}</p>
      </form>
      <br></br>
    </div>
  );
}

export default App;
