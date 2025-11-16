# Memorial Plaque Engraver - Complete Setup Guide

## Project Structure

```
memorial-plaque-engraver/
├── src/                          # Frontend (SvelteKit)
│   ├── routes/                  # SvelteKit routes
│   │   ├── +page.svelte        # Main application page
│   │   └── +layout.ts          # Layout config (SSR disabled)
│   ├── lib/                     # Library code
│   │   └── components/         # UI Components
│   │       ├── PersonalInfo.svelte  # Memorial information input
│   │       ├── DesignSettings.svelte # Layout and font settings
│   │       ├── CncSettings.svelte   # CNC parameters
│   │       ├── MachineControl.svelte # CNC control interface
│   │       ├── Preview.svelte       # Live preview canvas
│   │       └── StatusMessage.svelte # Toast notifications
│   └── app.html                # HTML template
├── src-tauri/                   # Backend (Rust/Tauri v2)
│   ├── src/
│   │   └── main.rs             # Tauri backend with CNC control
│   ├── capabilities/
│   │   └── main.json           # Tauri v2 permissions
│   ├── Cargo.toml              # Rust dependencies
│   └── tauri.conf.json         # Tauri v2 configuration
├── static/                      # Static assets
├── package.json                 # NPM dependencies
├── vite.config.js              # Vite configuration
├── svelte.config.js            # SvelteKit configuration
├── tsconfig.json               # TypeScript configuration
├── .gitignore                  # Git ignore rules
└── README.md                    # Project documentation
```

## Prerequisites

Before starting, ensure you have:

1. **Node.js** (v16 or higher)

    - Download from: https://nodejs.org/

2. **Rust** and **Cargo**

    - Install from: https://rustup.rs/
    - Run: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

3. **System Dependencies**
    - **Linux**: `sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libudev-dev`
    - **macOS**: `xcode-select --install`
    - **Windows**: Install Microsoft Visual Studio C++ Build Tools

## Installation Steps

### 1. Navigate to Project Directory

```bash
cd memorial-plaque-engraver
```

### 2. Install Frontend Dependencies

```bash
npm install
```

This will install:

-   SvelteKit framework
-   Svelte 5 compiler
-   Vite build tool
-   Tauri v2 API bindings
-   TypeScript support

### 3. Install Rust Dependencies

The Rust dependencies will be automatically installed when you first run the app. They include:

-   Tauri v2 framework
-   SerialPort library for CNC communication
-   Serde for JSON serialization

### 4. Development Mode

Run the application in development mode with hot-reload:

```bash
npm run tauri:dev
```

This will:

-   Start the Vite development server
-   Compile the Rust backend
-   Launch the application window
-   Enable hot-reload for both frontend and backend

### 5. Production Build

Build a production-ready executable:

```bash
npm run tauri:build
```

The built application will be in:

-   **Windows**: `src-tauri/target/release/memorial-plaque-engraver.exe`
-   **macOS**: `src-tauri/target/release/bundle/dmg/`
-   **Linux**: `src-tauri/target/release/bundle/deb/` or `src-tauri/target/release/bundle/appimage/`

## Application Usage

### Design Tab

1. **Personal Information**

    - Enter first and last name
    - Enter birth date (MM/DD/YYYY format)
    - Enter death date (MM/DD/YYYY format)

2. **Design Settings**

    - Choose a preset (Elegant, Modern, Classic, Bold)
    - Select layout style (Auto, Single Line, Stacked)
    - Adjust name size (5-12mm)
    - Adjust date size (4-10mm)

3. **CNC Settings**

    - Set cut depth (0.05-0.30mm, recommended 0.12-0.18mm)
    - Set feed rate (cutting speed)
    - Set plunge rate (entry speed)
    - Set spindle speed (8000-10000 RPM recommended)

4. **Preview**
    - View live preview of the engraving
    - Toggle grid for alignment reference
    - Zoom in/out for detailed inspection

### CNC Control Tab

1. **Machine Connection**

    - Select serial port from dropdown
    - Click "Connect to CNC" button
    - Wait for connection confirmation

2. **Machine Status**

    - View current machine position (X, Y, Z)
    - View work position relative to origin
    - Monitor engraving progress when running

3. **Machine Control**

    - **Home Machine**: Move to home position (required before engraving)
    - **Start Engraving**: Begin the engraving process
    - **Pause/Resume**: Pause and resume during engraving
    - **Stop**: Stop the current job
    - **Emergency Stop**: Immediately halt all movement

4. **Manual Jog**
    - Select jog distance (0.1, 1, 10, or 50mm)
    - Use directional buttons to move axes
    - Fine-tune position before starting

## Safety Guidelines

⚠️ **IMPORTANT SAFETY INFORMATION**

1. **Before Starting**

    - Secure material firmly to the work surface
    - Ensure correct bit is installed
    - Set proper Z-height (zero position)
    - Clear the work area of obstacles

2. **During Operation**

    - Never leave the machine unattended
    - Keep emergency stop accessible
    - Wear safety glasses
    - Monitor the first few passes closely

3. **Material Settings**

    - Start with shallow depth (0.10mm)
    - Test on scrap material first
    - Adjust settings based on material hardness
    - Recommended for aluminum: 0.12-0.18mm depth

4. **Emergency Procedures**
    - If something goes wrong, immediately press Emergency Stop
    - Power off the machine if necessary
    - Do not attempt to manually stop the spindle

## Troubleshooting

### Connection Issues

**Problem**: Cannot connect to CNC machine

-   Check USB cable connection
-   Verify correct serial port selected
-   Ensure no other software is using the port
-   Try unplugging and reconnecting the machine

**Problem**: Port not showing in list

-   Click refresh button
-   Check if drivers are installed
-   On Linux, add user to dialout group: `sudo usermod -a -G dialout $USER`

### Engraving Issues

**Problem**: Cuts are too deep

-   Reduce depth setting in CNC Settings
-   Verify Z-zero position is correct
-   Check material thickness

**Problem**: Machine not moving

-   Ensure machine is homed first
-   Check if machine is in alarm state
-   Verify connections are secure

**Problem**: Rough or uneven cuts

-   Reduce feed rate
-   Check if bit is sharp
-   Ensure material is flat and secured

### Software Issues

**Problem**: Preview not updating

-   Click "Update Preview" button
-   Check that all required fields are filled
-   Verify date format is correct (MM/DD/YYYY)

**Problem**: App won't start in development

-   Run `npm install` again
-   Clear node_modules: `rm -rf node_modules && npm install`
-   Check Rust is installed: `rustc --version`

## CNC Machine Compatibility

This application is designed for **GRBL-compatible CNC machines**, including:

-   VEVOR CNC 3018 Pro
-   CNC 3018-PRO
-   SainSmart Genmitsu 3018-PRO
-   Other GRBL v1.1+ controllers

**Serial Communication**: 115200 baud rate

## File Formats

### Settings Storage

-   Settings are automatically saved to: `~/.config/memorial-plaque-engraver/settings.json`
-   Includes personal info, design settings, and CNC parameters

### G-Code Output

-   G-Code is generated in real-time when starting engraving
-   Compatible with GRBL v1.1+
-   Metric units (mm)
-   Absolute positioning

## Font Information

The application uses a **Helvetica Clean** inspired font rendered as vector line segments:

-   Clean, professional appearance
-   Optimized for CNC engraving
-   Includes letters A-Z, numbers 0-9, space, hyphen, and slash

## Advanced Configuration

### Modifying Presets

Edit the presets in `src/App.svelte`:

```javascript
const presets = {
    elegant: {
        nameSize: 9,
        dateSize: 6,
        depth: 0.12,
        feedRate: 250,
        layout: "auto",
    },
    modern: {
        nameSize: 8,
        dateSize: 6,
        depth: 0.15,
        feedRate: 300,
        layout: "single",
    },
    classic: {
        nameSize: 10,
        dateSize: 7,
        depth: 0.18,
        feedRate: 200,
        layout: "stacked",
    },
    bold: {
        nameSize: 7,
        dateSize: 5,
        depth: 0.2,
        feedRate: 180,
        layout: "auto",
    },
};
```

### Adding Custom Fonts

Fonts are defined in `src/App.svelte` in the `cncFont` object. Each letter is defined as an array of line segments:

```javascript
'A': [[x1, y1, x2, y2], [x1, y1, x2, y2], ...],
```

## Development Tips

### Hot Reload

-   Frontend changes update automatically
-   Backend (Rust) changes require restart

### Debugging

-   Open DevTools: Press F12 in the application
-   View Rust logs in terminal where you ran `npm run tauri:dev`

### Building for Different Platforms

```bash
# Build for current platform
npm run tauri:build

# Build with debug symbols
npm run tauri:build -- --debug
```

## Support and Contribution

### Reporting Issues

-   Check existing issues first
-   Provide detailed description
-   Include error messages and screenshots
-   Specify operating system and version

### Contributing

-   Fork the repository
-   Create a feature branch
-   Make your changes
-   Submit a pull request

## License

MIT License - See LICENSE file for details

## Credits

-   Built with Svelte and Tauri
-   CNC control via GRBL protocol
-   Serial communication using Rust serialport crate

---

**Version**: 1.0.0  
**Last Updated**: 2024

For more information, visit the project repository.
