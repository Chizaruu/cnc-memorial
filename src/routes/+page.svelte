<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  import PersonalInfo from '$lib/components/PersonalInfo.svelte';
  import DesignSettings from '$lib/components/DesignSettings.svelte';
  import CncSettings from '$lib/components/CncSettings.svelte';
  import MachineControl from '$lib/components/MachineControl.svelte';
  import Preview from '$lib/components/Preview.svelte';
  import StatusMessage from '$lib/components/StatusMessage.svelte';

  // Reactive state using Svelte 5 runes
  let personalInfo = $state({
    firstName: 'John',
    lastName: 'Doe',
    birthDate: '01/15/1950',
    deathDate: '12/25/2023',
  });

  let designSettings = $state({
    layout: 'auto',
    nameSize: 8,
    dateSize: 6,
  });

  let cncSettings = $state({
    depth: 0.15,
    feedRate: 300,
    plungeRate: 150,
    spindleSpeed: 1000,
  });

  let machineState = $state({
    connected: false,
    port: '',
    status: 'Idle',
    position: { x: 0, y: 0, z: 0 },
    workPosition: { x: 0, y: 0, z: 0 },
    progress: 0,
    isHomed: false,
    isRunning: false,
    isPaused: false,
    availablePorts: [],
  });

  let uiState = $state({
    zoom: 1.0,
    showGrid: true,
    isGenerating: false,
    validationErrors: new Set(),
    activeTab: 'design', // 'design' or 'control'
  });

  let status = $state({
    message: '',
    type: '',
    visible: false,
  });

  // Derived state for complete settings
  let allSettings = $derived({
    ...personalInfo,
    ...designSettings,
    ...cncSettings,
  });

  // Validation derived state
  let isValid = $derived(
    personalInfo.firstName.trim() &&
      personalInfo.lastName.trim() &&
      personalInfo.birthDate.trim() &&
      personalInfo.deathDate.trim() &&
      uiState.validationErrors.size === 0
  );

  let previewRef = $state();

  // Font definition
  /**
   * @type {{ [key: string]: [number, number, number, number][] }}
   */
  const cncFont = {
    A: [
      [0, 0, 5, 14],
      [5, 14, 10, 0],
      [2.5, 5, 7.5, 5],
    ],
    B: [
      [0, 0, 0, 14],
      [0, 14, 6, 14],
      [0, 7, 6, 7],
      [0, 0, 6, 0],
      [6, 14, 8, 12],
      [8, 12, 8, 9],
      [8, 9, 6, 7],
      [6, 7, 8, 5],
      [8, 5, 8, 2],
      [8, 2, 6, 0],
    ],
    C: [
      [8, 12, 6, 14],
      [6, 14, 4, 14],
      [4, 14, 2, 12],
      [2, 12, 2, 2],
      [2, 2, 4, 0],
      [4, 0, 6, 0],
      [6, 0, 8, 2],
    ],
    D: [
      [0, 0, 0, 14],
      [0, 14, 6, 14],
      [6, 14, 8, 12],
      [8, 12, 8, 2],
      [8, 2, 6, 0],
      [6, 0, 0, 0],
    ],
    E: [
      [0, 0, 0, 14],
      [0, 14, 8, 14],
      [0, 7, 6, 7],
      [0, 0, 8, 0],
    ],
    F: [
      [0, 0, 0, 14],
      [0, 14, 8, 14],
      [0, 7, 6, 7],
    ],
    G: [
      [8, 12, 6, 14],
      [6, 14, 4, 14],
      [4, 14, 2, 12],
      [2, 12, 2, 2],
      [2, 2, 4, 0],
      [4, 0, 6, 0],
      [6, 0, 8, 2],
      [8, 2, 8, 6],
      [8, 6, 5, 6],
    ],
    H: [
      [0, 0, 0, 14],
      [8, 0, 8, 14],
      [0, 7, 8, 7],
    ],
    I: [[4, 0, 4, 14]],
    J: [
      [6, 14, 6, 3],
      [6, 3, 4, 0],
      [4, 0, 2, 0],
      [2, 0, 0, 2],
    ],
    K: [
      [0, 0, 0, 14],
      [0, 7, 8, 14],
      [0, 7, 8, 0],
    ],
    L: [
      [0, 0, 0, 14],
      [0, 0, 8, 0],
    ],
    M: [
      [0, 0, 0, 14],
      [0, 14, 5, 7],
      [5, 7, 10, 14],
      [10, 14, 10, 0],
    ],
    N: [
      [0, 0, 0, 14],
      [0, 14, 8, 0],
      [8, 0, 8, 14],
    ],
    O: [
      [4, 14, 2, 12],
      [2, 12, 2, 2],
      [2, 2, 4, 0],
      [4, 0, 6, 0],
      [6, 0, 8, 2],
      [8, 2, 8, 12],
      [8, 12, 6, 14],
      [6, 14, 4, 14],
    ],
    P: [
      [0, 0, 0, 14],
      [0, 14, 6, 14],
      [6, 14, 8, 12],
      [8, 12, 8, 9],
      [8, 9, 6, 7],
      [6, 7, 0, 7],
    ],
    Q: [
      [4, 14, 2, 12],
      [2, 12, 2, 2],
      [2, 2, 4, 0],
      [4, 0, 6, 0],
      [6, 0, 8, 2],
      [8, 2, 8, 12],
      [8, 12, 6, 14],
      [6, 14, 4, 14],
      [6, 3, 9, 0],
    ],
    R: [
      [0, 0, 0, 14],
      [0, 14, 6, 14],
      [6, 14, 8, 12],
      [8, 12, 8, 9],
      [8, 9, 6, 7],
      [6, 7, 0, 7],
      [4, 7, 8, 0],
    ],
    S: [
      [8, 11, 6, 14],
      [6, 14, 2, 14],
      [2, 14, 0, 12],
      [0, 12, 0, 9],
      [0, 9, 2, 7],
      [2, 7, 6, 7],
      [6, 7, 8, 5],
      [8, 5, 8, 2],
      [8, 2, 6, 0],
      [6, 0, 2, 0],
      [2, 0, 0, 3],
    ],
    T: [
      [0, 14, 10, 14],
      [5, 14, 5, 0],
    ],
    U: [
      [0, 14, 0, 3],
      [0, 3, 2, 0],
      [2, 0, 6, 0],
      [6, 0, 8, 3],
      [8, 3, 8, 14],
    ],
    V: [
      [0, 14, 5, 0],
      [5, 0, 10, 14],
    ],
    W: [
      [0, 14, 2, 0],
      [2, 0, 4, 7],
      [4, 7, 6, 0],
      [6, 0, 8, 14],
    ],
    X: [
      [0, 14, 8, 0],
      [0, 0, 8, 14],
    ],
    Y: [
      [0, 14, 4, 7],
      [8, 14, 4, 7],
      [4, 7, 4, 0],
    ],
    Z: [
      [0, 14, 8, 14],
      [8, 14, 0, 0],
      [0, 0, 8, 0],
    ],
    ' ': [],
    '-': [[2, 7, 6, 7]],
    '/': [[2, 0, 6, 14]],
    '0': [
      [4, 14, 2, 12],
      [2, 12, 2, 2],
      [2, 2, 4, 0],
      [4, 0, 6, 0],
      [6, 0, 8, 2],
      [8, 2, 8, 12],
      [8, 12, 6, 14],
      [6, 14, 4, 14],
    ],
    '1': [
      [3, 12, 4, 14],
      [4, 14, 4, 0],
      [2, 0, 6, 0],
    ],
    '2': [
      [2, 12, 4, 14],
      [4, 14, 6, 14],
      [6, 14, 8, 12],
      [8, 12, 8, 8],
      [8, 8, 0, 0],
      [0, 0, 8, 0],
    ],
    '3': [
      [0, 12, 2, 14],
      [2, 14, 6, 14],
      [6, 14, 8, 12],
      [8, 12, 8, 8],
      [8, 8, 6, 7],
      [6, 7, 4, 7],
      [6, 7, 8, 6],
      [8, 6, 8, 2],
      [8, 2, 6, 0],
      [6, 0, 2, 0],
      [2, 0, 0, 2],
    ],
    '4': [
      [6, 14, 6, 0],
      [6, 14, 0, 4],
      [0, 4, 8, 4],
    ],
    '5': [
      [8, 14, 0, 14],
      [0, 14, 0, 8],
      [0, 8, 6, 8],
      [6, 8, 8, 6],
      [8, 6, 8, 2],
      [8, 2, 6, 0],
      [6, 0, 2, 0],
      [2, 0, 0, 2],
    ],
    '6': [
      [6, 14, 4, 14],
      [4, 14, 2, 12],
      [2, 12, 2, 2],
      [2, 2, 4, 0],
      [4, 0, 6, 0],
      [6, 0, 8, 2],
      [8, 2, 8, 6],
      [8, 6, 6, 8],
      [6, 8, 2, 8],
    ],
    '7': [
      [0, 14, 8, 14],
      [8, 14, 3, 0],
    ],
    '8': [
      [4, 14, 2, 12],
      [2, 12, 2, 9],
      [2, 9, 4, 7],
      [4, 7, 6, 7],
      [6, 7, 8, 9],
      [8, 9, 8, 12],
      [8, 12, 6, 14],
      [6, 14, 4, 14],
      [4, 7, 2, 5],
      [2, 5, 2, 2],
      [2, 2, 4, 0],
      [4, 0, 6, 0],
      [6, 0, 8, 2],
      [8, 2, 8, 5],
      [8, 5, 6, 7],
    ],
    '9': [
      [2, 0, 4, 0],
      [4, 0, 6, 2],
      [6, 2, 6, 12],
      [6, 12, 4, 14],
      [4, 14, 2, 14],
      [2, 14, 0, 12],
      [0, 12, 0, 8],
      [0, 8, 2, 6],
      [2, 6, 6, 6],
    ],
  };

  // Utility functions
  /**
   * @param {string} dateString
   */
  function extractYear(dateString) {
    if (!dateString) return '';
    const yearMatch = dateString.match(/\d{4}/);
    return yearMatch ? yearMatch[0] : dateString;
  }

  /**
   * @param {string} message
   */
  function showStatus(message, type = 'info') {
    status.message = message;
    status.type = type;
    status.visible = true;

    setTimeout(
      () => {
        status.visible = false;
      },
      type === 'success' ? 4000 : 6000
    );
  }

  /**
   * @param {string} presetName
   */
  function loadPreset(presetName) {
    /**
     * @type {{ [key: string]: { nameSize: number; dateSize: number; depth: number; feedRate: number; layout: string; } }}
     */
    const presets = {
      elegant: { nameSize: 9, dateSize: 6, depth: 0.12, feedRate: 250, layout: 'auto' },
      modern: { nameSize: 8, dateSize: 6, depth: 0.15, feedRate: 300, layout: 'single' },
      classic: { nameSize: 10, dateSize: 7, depth: 0.18, feedRate: 200, layout: 'stacked' },
      bold: { nameSize: 7, dateSize: 5, depth: 0.2, feedRate: 180, layout: 'auto' },
    };

    const preset = presets[presetName];
    if (preset) {
      designSettings.nameSize = preset.nameSize;
      designSettings.dateSize = preset.dateSize;
      designSettings.layout = preset.layout;

      cncSettings.depth = preset.depth;
      cncSettings.feedRate = preset.feedRate;

      showStatus(
        `${presetName.charAt(0).toUpperCase() + presetName.slice(1)} preset loaded!`,
        'success'
      );
    }
  }

  function updatePreview() {
    if (previewRef) {
      previewRef.updateCanvas(allSettings, uiState);
    }
  }

  // CNC Machine Control Functions
  /**
   * @param {string} port
   */
  async function connectToMachine(port) {
    try {
      await invoke('connect_cnc', { port });
      machineState.connected = true;
      machineState.port = port;
      showStatus(`Connected to CNC machine on ${port}!`, 'success');

      startStatusPolling();
    } catch (error) {
      showStatus(`Failed to connect: ${error}`, 'error');
    }
  }

  async function disconnectMachine() {
    try {
      await invoke('disconnect_cnc');
      machineState.connected = false;
      machineState.port = '';
      machineState.status = 'Idle';
      machineState.isRunning = false;
      machineState.isPaused = false;
      showStatus('Disconnected from CNC machine', 'info');
    } catch (error) {
      showStatus(`Failed to disconnect: ${error}`, 'error');
    }
  }

  async function homeMachine() {
    try {
      await invoke('home_machine');
      showStatus('Homing machine...', 'info');
    } catch (error) {
      showStatus(`Homing failed: ${error}`, 'error');
    }
  }

  /**
   * @param {any} axis
   * @param {any} distance
   */
  async function jogMachine(axis, distance) {
    try {
      await invoke('jog_machine', { axis, distance });
    } catch (error) {
      showStatus(`Jog failed: ${error}`, 'error');
    }
  }

  async function startEngraving() {
    if (!isValid) {
      showStatus('Please fix validation errors first!', 'error');
      return;
    }

    if (!machineState.connected) {
      showStatus('Please connect to CNC machine first!', 'error');
      return;
    }

    if (!machineState.isHomed) {
      showStatus('Please home the machine first!', 'error');
      return;
    }

    try {
      const gcode = createGCodeOutput(allSettings);
      await invoke('start_engraving', { gcode });
      machineState.isRunning = true;
      machineState.isPaused = false;
      showStatus('Engraving started!', 'success');
    } catch (error) {
      showStatus(`Failed to start engraving: ${error}`, 'error');
    }
  }

  async function pauseEngraving() {
    try {
      await invoke('pause_engraving');
      machineState.isPaused = true;
      showStatus('Engraving paused', 'info');
    } catch (error) {
      showStatus(`Failed to pause: ${error}`, 'error');
    }
  }

  async function resumeEngraving() {
    try {
      await invoke('resume_engraving');
      machineState.isPaused = false;
      showStatus('Engraving resumed', 'info');
    } catch (error) {
      showStatus(`Failed to resume: ${error}`, 'error');
    }
  }

  async function stopEngraving() {
    try {
      await invoke('stop_engraving');
      machineState.isRunning = false;
      machineState.isPaused = false;
      machineState.progress = 0;
      showStatus('Engraving stopped', 'info');
    } catch (error) {
      showStatus(`Failed to stop: ${error}`, 'error');
    }
  }

  async function emergencyStop() {
    try {
      await invoke('emergency_stop');
      machineState.isRunning = false;
      machineState.isPaused = false;
      machineState.progress = 0;
      machineState.status = 'Alarm';
      showStatus('EMERGENCY STOP activated!', 'error');
    } catch (error) {
      console.error('Emergency stop failed:', error);
    }
  }

  async function refreshPorts() {
    try {
      const ports = await invoke('get_available_ports');
      machineState.availablePorts = ports;
    } catch (error) {
      showStatus(`Failed to refresh ports: ${error}`, 'error');
    }
  }

  function startStatusPolling() {
    if (!machineState.connected) return;

    const pollStatus = async () => {
      try {
        const statusInfo = await invoke('get_machine_status');
        machineState.status = statusInfo.status;
        machineState.position = statusInfo.machine_position;
        machineState.workPosition = statusInfo.work_position;
        machineState.progress = statusInfo.progress;
        machineState.isHomed = statusInfo.is_homed;
      } catch (error) {
        console.warn('Status polling error:', error);
      }

      if (machineState.connected) {
        setTimeout(pollStatus, 500);
      }
    };

    pollStatus();
  }

  // G-code generation
  /**
   * @param {{ depth: number; feedRate: number; plungeRate: number; spindleSpeed: number; layout: string; nameSize: number; dateSize: number; firstName: string; lastName: string; birthDate: string; deathDate: string; }} settings
   */
  function createGCodeOutput(settings) {
    const lines = [];
    lines.push(...createGCodeHeader(settings));
    lines.push(...createInitializationSequence(settings));
    lines.push(...generateToolpaths(settings));
    lines.push(...createShutdownSequence());
    return lines.join('\n');
  }

  /**
   * @param {{ birthDate: any; deathDate: any; firstName: any; lastName: any; layout: any; nameSize: any; dateSize: any; depth: any; feedRate: any; plungeRate: any; spindleSpeed: any; }} settings
   */
  function createGCodeHeader(settings) {
    const timestamp = new Date().toISOString();
    const birthYear = extractYear(settings.birthDate);
    const deathYear = extractYear(settings.deathDate);

    return [
      '; ============================================',
      '; Memorial Plaque G-Code - Helvetica Clean',
      '; ============================================',
      `; Generated: ${timestamp}`,
      `; Name: ${settings.firstName} ${settings.lastName}`,
      `; Years: ${birthYear} - ${deathYear}`,
      `; Birth Date: ${settings.birthDate}`,
      `; Death Date: ${settings.deathDate}`,
      `; Font: Helvetica Clean`,
      `; Layout: ${settings.layout}`,
      `; Name Size: ${settings.nameSize}mm`,
      `; Year Size: ${settings.dateSize}mm`,
      `; Cut Depth: ${settings.depth}mm`,
      `; Feed Rate: ${settings.feedRate}mm/min`,
      `; Plunge Rate: ${settings.plungeRate}mm/min`,
      `; Spindle Speed: ${settings.spindleSpeed}RPM`,
      '; ============================================',
      '',
    ];
  }

  /**
   * @param {{ spindleSpeed: any; }} settings
   */
  function createInitializationSequence(settings) {
    return [
      'G21 ; Set units to millimeters',
      'G90 ; Absolute positioning',
      'G17 ; Select XY plane',
      'G94 ; Feed rate per minute',
      'G54 ; Work coordinate system',
      '',
      '; Safety and setup',
      'G0 Z10 ; Raise Z to safe height',
      'G0 X0 Y0 ; Move to origin',
      '',
      `M3 S${settings.spindleSpeed} ; Start spindle`,
      'G4 P3 ; Wait 3 seconds for spindle to reach speed',
      '',
    ];
  }

  /**
   * @param {any} settings
   */
  function generateToolpaths(settings) {
    /**
     * @type {string[]}
     */
    const lines = [];
    const layout = calculateOptimalLayout(settings);

    layout.lines.forEach((line, index) => {
      const lineType =
        index === layout.lines.length - 1
          ? 'Years'
          : layout.lines.length === 3 && index === 1
            ? 'Last Name'
            : layout.lines.length === 3 && index === 0
              ? 'First Name'
              : 'Full Name';
      lines.push(`; ${lineType}: ${line.text}`);
      lines.push(...generateTextLineGCode(line.text, line.x, line.y, line.size, settings));
      lines.push('');
    });

    return lines;
  }

  /**
   * @param {string | any[]} text
   * @param {number} centerX
   * @param {number} y
   * @param {number} fontSize
   * @param {any} settings
   */
  function generateTextLineGCode(text, centerX, y, fontSize, settings) {
    const lines = [];
    const charSpacing = fontSize * 1.2;
    const totalWidth = (text.length - 1) * charSpacing;
    const startX = centerX / 2 - totalWidth / 2;

    for (let i = 0; i < text.length; i++) {
      const char = text[i];
      const charX = startX + i * charSpacing;
      const charY = (180 - y) / 2;

      lines.push(`; Character: '${char}'`);
      lines.push(...generateLetterGCode(char, charX, charY, fontSize, settings));
    }

    return lines;
  }

  /**
   * @param {string | number} letter
   * @param {number} x
   * @param {number} y
   * @param {number} size
   * @param {{ depth: any; feedRate: any; plungeRate: any; }} settings
   */
  function generateLetterGCode(letter, x, y, size, settings) {
    const letterDef = cncFont[letter];
    if (!letterDef || letterDef.length === 0) return [];

    /**
     * @type {string[]}
     */
    const lines = [];
    const scaleFactor = size / 14;
    const { depth, feedRate, plungeRate } = settings;

    letterDef.forEach((/** @type {[any, any, any, any]} */ line) => {
      const [x1, y1, x2, y2] = line;
      const scaledX1 = (x + x1 * scaleFactor).toFixed(3);
      const scaledY1 = (y + y1 * scaleFactor).toFixed(3);
      const scaledX2 = (x + x2 * scaleFactor).toFixed(3);
      const scaledY2 = (y + y2 * scaleFactor).toFixed(3);

      lines.push(`G0 X${scaledX1} Y${scaledY1} ; Move to start`);
      lines.push(`G1 Z-${depth} F${plungeRate} ; Plunge`);
      lines.push(`G1 X${scaledX2} Y${scaledY2} F${feedRate} ; Cut line`);
      lines.push('G0 Z2 ; Retract');
    });

    return lines;
  }

  function createShutdownSequence() {
    return [
      '; Shutdown sequence',
      'G0 Z10 ; Raise Z to safe height',
      'G0 X0 Y0 ; Return to origin',
      'M5 ; Stop spindle',
      'M30 ; Program end and reset',
    ];
  }

  /**
   * @param {{ firstName: string; lastName: string; birthDate: any; deathDate: any; nameSize: any; dateSize: any; layout: string; }} settings
   */
  function calculateOptimalLayout(settings) {
    const firstName = settings.firstName.trim().toUpperCase();
    const lastName = settings.lastName.trim().toUpperCase();
    const fullName = `${firstName} ${lastName}`;

    const birthYear = extractYear(settings.birthDate);
    const deathYear = extractYear(settings.deathDate);
    const yearText = `${birthYear} - ${deathYear}`;

    const nameSize = settings.nameSize;
    const dateSize = settings.dateSize;
    const centerX = 180;

    let lines = [];

    const estimatedNameWidth = fullName.length * nameSize * 0.9 * uiState.zoom;
    const maxWidth = 160 * 2 * uiState.zoom;

    const useStacked =
      settings.layout === 'stacked' ||
      (settings.layout === 'auto' && estimatedNameWidth > maxWidth);

    let nameStartY = 180 * 0.25;

    if (useStacked) {
      lines.push({
        text: firstName,
        x: centerX,
        y: nameStartY,
        size: nameSize,
      });

      lines.push({
        text: lastName,
        x: centerX,
        y: nameStartY + nameSize * 2 * 1.4,
        size: nameSize,
      });
    } else {
      lines.push({
        text: fullName,
        x: centerX,
        y: nameStartY + nameSize * 2 * 0.7,
        size: nameSize,
      });
    }

    const bottomMargin = 25 * 2;
    lines.push({
      text: yearText,
      x: centerX,
      y: 180 - bottomMargin,
      size: dateSize,
    });

    return { lines };
  }

  // Auto-update preview when settings change
  $effect(() => {
    if (previewRef) {
      updatePreview();
    }
  });

  // Auto-save settings when they change
  $effect(() => {
    invoke('save_settings', {
      settings: {
        personal: personalInfo,
        design: designSettings,
        cnc: cncSettings,
      },
    }).catch(() => {});
  });

  onMount(() => {
    // Load saved settings
    invoke('load_settings')
      .then((savedSettings) => {
        if (savedSettings) {
          Object.assign(personalInfo, savedSettings.personal || personalInfo);
          Object.assign(designSettings, savedSettings.design || designSettings);
          Object.assign(cncSettings, savedSettings.cnc || cncSettings);
        }
      })
      .catch(() => {});

    // Refresh available ports
    refreshPorts();
  });
</script>

<main class="app">
  <header class="header">
    <div class="header-content">
      <h1>⚱️ Memorial Plaque Engraver</h1>
      <p>Professional VEVOR CNC 3018 Pro Control - Helvetica Clean Font - 300×180×45mm</p>
    </div>

    <div class="tab-navigation">
      <button
        class="tab-btn"
        class:active={uiState.activeTab === 'design'}
        onclick={() => (uiState.activeTab = 'design')}
      >
        🎨 Design
      </button>
      <button
        class="tab-btn"
        class:active={uiState.activeTab === 'control'}
        onclick={() => (uiState.activeTab = 'control')}
      >
        🔧 CNC Control
      </button>
    </div>
  </header>

  <div class="main-content">
    {#if uiState.activeTab === 'design'}
      <!-- Design Tab -->
      <div class="left-column">
        <PersonalInfo bind:personalInfo />
        <DesignSettings bind:designSettings {loadPreset} />
        <CncSettings bind:cncSettings />
      </div>

      <div class="right-column">
        <div class="preview-header">
          <h2>Live Preview</h2>
          <p>Helvetica Clean font with names centered and years at bottom</p>
        </div>

        <Preview
          bind:this={previewRef}
          {allSettings}
          {uiState}
          {cncFont}
          {calculateOptimalLayout}
        />

        <div class="action-buttons">
          <button class="btn btn-primary" onclick={updatePreview} disabled={uiState.isGenerating}>
            🔄 Update Preview
          </button>
          <button
            class="btn btn-success"
            onclick={startEngraving}
            disabled={!isValid || !machineState.connected || machineState.isRunning}
          >
            {#if !machineState.connected}
              📡 Connect CNC First
            {:else if !isValid}
              ⚠️ Fix Errors First
            {:else if machineState.isRunning}
              🔄 Engraving...
            {:else}
              🔥 Start Engraving
            {/if}
          </button>
        </div>
      </div>
    {:else}
      <!-- CNC Control Tab -->
      <MachineControl
        bind:machineState
        {connectToMachine}
        {disconnectMachine}
        {homeMachine}
        {jogMachine}
        {startEngraving}
        {pauseEngraving}
        {resumeEngraving}
        {stopEngraving}
        {emergencyStop}
        {refreshPorts}
        {isValid}
        {allSettings}
      />
    {/if}
  </div>

  <StatusMessage {status} />
</main>

<style>
  :global(*) {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
  }

  :global(body) {
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
    color: #333;
    line-height: 1.6;
    height: 100vh;
    overflow: hidden;
  }

  .app {
    height: 100vh;
    display: flex;
    flex-direction: column;
    background: white;
  }

  .header {
    background: linear-gradient(135deg, #0f3460, #16537e);
    color: white;
    padding: 20px 30px;
    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .header-content h1 {
    font-size: 2.2em;
    font-weight: 300;
    margin-bottom: 5px;
  }

  .header-content p {
    opacity: 0.9;
    font-size: 1em;
  }

  .tab-navigation {
    display: flex;
    gap: 10px;
  }

  .tab-btn {
    padding: 12px 20px;
    background: rgba(255, 255, 255, 0.1);
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-radius: 8px;
    color: white;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .tab-btn:hover {
    background: rgba(255, 255, 255, 0.2);
    transform: translateY(-2px);
  }

  .tab-btn.active {
    background: rgba(255, 255, 255, 0.3);
    border-color: rgba(255, 255, 255, 0.5);
    box-shadow: 0 4px 12px rgba(255, 255, 255, 0.2);
  }

  .main-content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .left-column {
    width: 400px;
    background: #f8f9fa;
    border-right: 1px solid #e9ecef;
    overflow-y: auto;
    padding: 30px;
  }

  .right-column {
    flex: 1;
    padding: 30px;
    background: linear-gradient(135deg, #ffffff, #f1f2f6);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .preview-header {
    background: linear-gradient(135deg, #3498db, #2980b9);
    color: white;
    padding: 20px;
    border-radius: 12px;
    margin-bottom: 20px;
    text-align: center;
  }

  .preview-header h2 {
    font-size: 1.5em;
    font-weight: 300;
    margin-bottom: 5px;
  }

  .preview-header p {
    opacity: 0.9;
    font-size: 0.95em;
  }

  .action-buttons {
    display: flex;
    gap: 15px;
    justify-content: center;
    margin-top: 20px;
  }

  .btn {
    padding: 15px 25px;
    border: none;
    border-radius: 8px;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    min-width: 160px;
  }

  .btn-primary {
    background: linear-gradient(135deg, #3498db, #2980b9);
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 8px 20px rgba(52, 152, 219, 0.4);
  }

  .btn-success {
    background: linear-gradient(135deg, #e74c3c, #c0392b);
    color: white;
  }

  .btn-success:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 8px 20px rgba(231, 76, 60, 0.4);
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    transform: none !important;
  }

  @media (max-width: 1024px) {
    .header {
      flex-direction: column;
      gap: 15px;
      text-align: center;
    }

    .main-content {
      flex-direction: column;
    }

    .left-column {
      width: 100%;
      order: 2;
      max-height: 50vh;
    }

    .right-column {
      order: 1;
      min-height: 50vh;
    }

    .action-buttons {
      flex-direction: column;
      align-items: center;
    }

    .btn {
      width: 100%;
      max-width: 300px;
    }
  }
</style>
