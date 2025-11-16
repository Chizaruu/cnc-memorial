# Memorial Plaque Engraver

Professional memorial plaque engraving application with CNC 3018 Pro control, built with Svelte and Tauri.

## Features

- 🎨 Interactive design interface with live preview
- ⚙️ Full CNC machine control (GRBL compatible)
- 📝 Helvetica Clean font rendering
- 🔧 Manual jogging and positioning
- 🚨 Emergency stop functionality
- 💾 Auto-save settings
- 📊 Real-time machine status monitoring

## Prerequisites

- Node.js (v16 or higher)
- Rust and Cargo
- VEVOR CNC 3018 Pro or compatible GRBL-based CNC machine

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd memorial-plaque-engraver
```

2. Install frontend dependencies:
```bash
npm install
```

3. Build and run:
```bash
npm run tauri dev
```

## Building for Production

```bash
npm run tauri build
```

## Project Structure

```
memorial-plaque-engraver/
├── src/                    # Svelte frontend
│   ├── components/        # UI components
│   ├── App.svelte        # Main application
│   └── main.js           # Entry point
├── src-tauri/            # Rust backend
│   ├── src/
│   │   └── main.rs      # Tauri backend logic
│   ├── Cargo.toml       # Rust dependencies
│   └── tauri.conf.json  # Tauri configuration
└── public/              # Static assets
```

## Usage

1. **Design Tab**: Enter memorial information and adjust design settings
2. **CNC Control Tab**: Connect to your CNC machine and control engraving
3. Review the live preview before starting
4. Home the machine before engraving
5. Start the engraving process

## Safety

- Always home the machine before starting
- Keep emergency stop accessible
- Monitor the engraving process
- Ensure proper material securing

## License

MIT
