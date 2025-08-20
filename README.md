# ⚡️ Fluence Calculator Desktop App 🚀

A blazing-fast desktop app for calculating laser **fluence** and **pulse energy**!  
Built with [Tauri](https://tauri.app/) 🦀, [React](https://react.dev/) ⚛️, and [TypeScript](https://www.typescriptlang.org/) 🟦.

---

## ✨ Features

- 🖥️ **Cross-platform:** Works on Windows, macOS, and Linux!
- ⚡ **Instant Results:** Calculates pulse energy & fluence in real time.
- 🎯 **Simple UI:** Just enter your laser parameters and hit "Calculate".
- 🦀 **Rust-powered Backend:** Super fast and lightweight.

---

## 🛠️ Getting Started

1. **Install dependencies:**  
   ```sh
   npm install
   ```

2. **Run in development mode:**  
   ```sh
   npm run tauri dev
   ```

3. **Build for release:**  
   ```sh
   npm run tauri build
   ```

---

## 📥 Inputs

- **Power (mW)**
- **Pulse Picker**
- **Rep Rate (kHz)**
- **Spot Diameter (mm)**

## 📤 Output

- **Pulse Energy**
- **Fluence**

---

## 🖼️ UI Preview

The app features a clean form for entering laser parameters.  
On clicking **Calculate**, the results for **Pulse Energy** and **Fluence** are instantly displayed below the button.

```tsx
// Example from src/App.tsx
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
  <br />
  <button type="submit">Calculate</button>
  <p id="energy-val">{energyMsg}</p>
  <p id="fluence-val">{fluenceMsg}</p>
</form>
```

---

## 🧑‍💻 Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) 💻
- [Tauri VS Code Extension](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) 🦀
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) 🦀

---

## 📁 Project Structure

```
/src-tauri      # Rust backend (Tauri commands)
/src            # React frontend
```

---

## 📜 License

MIT

---

Made with ❤️ using Tauri, React, and Rust.  
Let the photons flow! ✨🔬