# Quick Start Guide

## Installation (5 minutes)

```bash
# 1. Install dependencies
npm install

# 2. Run the app
npm run tauri:dev
```

## First Time Setup

1. **Connect CNC Machine**

    - Plug in CNC via USB
    - Go to "CNC Control" tab
    - Select serial port
    - Click "Connect to CNC"

2. **Home the Machine**

    - Click "Home Machine" button
    - Wait for homing to complete
    - Verify "✅ Homed" status shows

3. **Design Your Plaque**

    - Go to "Design" tab
    - Enter memorial information
    - Choose a preset or customize settings
    - Review the live preview

4. **Start Engraving**
    - Ensure material is secured
    - Click "Start Engraving"
    - Monitor progress
    - Use Emergency Stop if needed

## Common Commands

| Action           | Command               |
| ---------------- | --------------------- |
| Start dev mode   | `npm run tauri:dev`   |
| Build production | `npm run tauri:build` |
| Install deps     | `npm install`         |

## Default Settings

-   **Depth**: 0.15mm
-   **Feed Rate**: 300mm/min
-   **Spindle Speed**: 1000 RPM
-   **Plunge Rate**: 150mm/min

## Emergency Procedures

**If something goes wrong:**

1. Click "EMERGENCY STOP" button immediately
2. Power off the machine if necessary
3. Never manually interfere with moving parts

## Keyboard Shortcuts

-   **F12**: Open developer tools
-   **Ctrl+R**: Refresh (dev mode only)

## Quick Tips

✅ **DO**:

-   Test on scrap material first
-   Start with shallow depth
-   Home before each job
-   Secure material firmly

❌ **DON'T**:

-   Leave machine unattended
-   Exceed recommended speeds
-   Skip the homing step
-   Touch moving parts

## Support

-   Check `SETUP_GUIDE.md` for detailed instructions
-   Review `README.md` for project overview
-   Check troubleshooting section for common issues

---

**Remember**: Safety first! Always monitor your CNC machine during operation.
