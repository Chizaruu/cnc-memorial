<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import * as Tabs from '$lib/components/ui/tabs';

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
    activeTab: 'design',
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

  // Font definition (keeping existing CNC font)
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
  function extractYear(dateString) {
    if (!dateString) return '';
    const yearMatch = dateString.match(/\d{4}/);
    return yearMatch ? yearMatch[0] : dateString;
  }

  function showStatus(message, type = 'info') {
    status.message = message;
    status.type = type;
    status.visible = true;

    setTimeout(
      () => {
        status.visible = false;
      },
      type === 'error' ? 5000 : 3000
    );
  }

  // Tool path functions (keeping existing logic)
  function loadPreset(presetName) {
    const presets = {
      elegant: { layout: 'auto', nameSize: 7, dateSize: 5 },
      modern: { layout: 'single', nameSize: 9, dateSize: 6 },
      classic: { layout: 'stacked', nameSize: 8, dateSize: 5.5 },
      bold: { layout: 'single', nameSize: 11, dateSize: 7 },
    };

    const preset = presets[presetName];
    if (preset) {
      Object.assign(designSettings, preset);
      showStatus(`Loaded ${presetName} preset`, 'success');
    }
  }

  // CNC Functions (keeping existing implementations)
  function updatePreview() {
    if (previewRef && previewRef.updateCanvas) {
      previewRef.updateCanvas();
      showStatus('Preview updated', 'info');
    }
  }

  async function connectToMachine(port) {
    try {
      uiState.isGenerating = true;
      await invoke('connect_to_machine', { port });
      machineState.connected = true;
      machineState.port = port;
      showStatus(`Connected to ${port}`, 'success');

      const statusInterval = setInterval(async () => {
        if (!machineState.connected) {
          clearInterval(statusInterval);
          return;
        }
        try {
          const status = await invoke('get_machine_status');
          Object.assign(machineState, status);
        } catch (error) {
          console.error('Status update error:', error);
        }
      }, 100);
    } catch (error) {
      showStatus(`Connection failed: ${error}`, 'error');
    } finally {
      uiState.isGenerating = false;
    }
  }

  async function disconnectMachine() {
    try {
      await invoke('disconnect_machine');
      machineState.connected = false;
      machineState.port = '';
      showStatus('Disconnected from machine', 'info');
    } catch (error) {
      showStatus(`Disconnect failed: ${error}`, 'error');
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

  async function jogMachine(axis, distance) {
    try {
      await invoke('jog_machine', { axis, distance });
    } catch (error) {
      showStatus(`Jog failed: ${error}`, 'error');
    }
  }

  async function startEngraving() {
    if (!isValid) {
      showStatus('Please fill in all required fields', 'error');
      return;
    }

    if (!machineState.connected) {
      showStatus('Please connect to CNC machine first', 'error');
      return;
    }

    try {
      uiState.isGenerating = true;
      const gcode = await invoke('generate_gcode', { settings: allSettings });
      await invoke('start_engraving', { gcode });
      showStatus('Engraving started', 'success');
    } catch (error) {
      showStatus(`Engraving failed: ${error}`, 'error');
    } finally {
      uiState.isGenerating = false;
    }
  }

  async function pauseEngraving() {
    try {
      await invoke('pause_engraving');
      showStatus('Engraving paused', 'info');
    } catch (error) {
      showStatus(`Pause failed: ${error}`, 'error');
    }
  }

  async function resumeEngraving() {
    try {
      await invoke('resume_engraving');
      showStatus('Engraving resumed', 'info');
    } catch (error) {
      showStatus(`Resume failed: ${error}`, 'error');
    }
  }

  async function stopEngraving() {
    try {
      await invoke('stop_engraving');
      showStatus('Engraving stopped', 'warning');
    } catch (error) {
      showStatus(`Stop failed: ${error}`, 'error');
    }
  }

  async function emergencyStop() {
    try {
      await invoke('emergency_stop');
      showStatus('EMERGENCY STOP ACTIVATED', 'error');
    } catch (error) {
      showStatus(`Emergency stop failed: ${error}`, 'error');
    }
  }

  async function refreshPorts() {
    try {
      const ports = await invoke('list_serial_ports');
      machineState.availablePorts = ports;
      showStatus('Ports refreshed', 'info');
    } catch (error) {
      showStatus(`Failed to refresh ports: ${error}`, 'error');
    }
  }

  function calculateOptimalLayout(settings) {
    const firstName = settings.firstName || '';
    const lastName = settings.lastName || '';
    const fullName = `${firstName} ${lastName}`.trim();
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

<main class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
  <div class="flex h-screen flex-col">
    <!-- Header -->
    <header
      class="border-b border-slate-700 bg-gradient-to-r from-slate-800 to-slate-900 px-6 py-4 shadow-xl"
    >
      <div class="mx-auto flex max-w-screen-2xl items-center justify-between">
        <div class="space-y-1">
          <h1 class="text-3xl font-bold text-white flex items-center gap-2">
            <span class="text-4xl">⚱️</span>
            Memorial Plaque Engraver
          </h1>
          <p class="text-sm text-slate-300">
            Professional VEVOR CNC 3018 Pro Control - Helvetica Clean Font - 300×180×45mm
          </p>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <div class="flex-1 overflow-hidden">
      <Tabs.Root value={uiState.activeTab} class="h-full flex flex-col">
        <Tabs.List class="w-full border-b border-slate-700 bg-slate-800/50 px-6">
          <Tabs.Trigger
            value="design"
            onclick={() => (uiState.activeTab = 'design')}
            class="data-[state=active]:bg-slate-700"
          >
            <span class="text-lg mr-2">🎨</span>
            Design
          </Tabs.Trigger>
          <Tabs.Trigger
            value="control"
            onclick={() => (uiState.activeTab = 'control')}
            class="data-[state=active]:bg-slate-700"
          >
            <span class="text-lg mr-2">🔧</span>
            CNC Control
          </Tabs.Trigger>
        </Tabs.List>

        <Tabs.Content value="design" class="flex-1 overflow-hidden mt-0">
          <div class="flex h-full">
            <!-- Left Sidebar -->
            <div
              class="w-96 overflow-y-auto border-r border-slate-700 bg-slate-800/30 p-6 space-y-6"
            >
              <PersonalInfo bind:personalInfo />
              <DesignSettings bind:designSettings {loadPreset} />
              <CncSettings bind:cncSettings />
            </div>

            <!-- Right Preview Area -->
            <div class="flex-1 overflow-y-auto bg-slate-900/50 p-6">
              <div class="mx-auto max-w-4xl space-y-6">
                <div
                  class="rounded-lg bg-gradient-to-br from-blue-500 to-blue-600 p-6 text-center text-white shadow-xl"
                >
                  <h2 class="text-2xl font-semibold">Live Preview</h2>
                  <p class="mt-1 text-blue-100">
                    Helvetica Clean font with names centered and years at bottom
                  </p>
                </div>

                <Preview
                  bind:this={previewRef}
                  {allSettings}
                  {uiState}
                  {cncFont}
                  {calculateOptimalLayout}
                />

                <div class="flex justify-center gap-4">
                  <button
                    class="inline-flex h-11 items-center justify-center rounded-md bg-blue-600 px-8 py-2 text-sm font-semibold text-white shadow-lg transition-all hover:bg-blue-700 hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-blue-500 disabled:pointer-events-none disabled:opacity-50"
                    onclick={updatePreview}
                    disabled={uiState.isGenerating}
                  >
                    <span class="mr-2">🔄</span>
                    Update Preview
                  </button>
                  <button
                    class="inline-flex h-11 items-center justify-center rounded-md bg-red-600 px-8 py-2 text-sm font-semibold text-white shadow-lg transition-all hover:bg-red-700 hover:shadow-xl focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-red-500 disabled:pointer-events-none disabled:opacity-50"
                    onclick={startEngraving}
                    disabled={!isValid || !machineState.connected || machineState.isRunning}
                  >
                    {#if !machineState.connected}
                      <span class="mr-2">📡</span>
                      Connect CNC First
                    {:else if !isValid}
                      <span class="mr-2">⚠️</span>
                      Fix Errors First
                    {:else if machineState.isRunning}
                      <span class="mr-2">🔄</span>
                      Engraving...
                    {:else}
                      <span class="mr-2">🔥</span>
                      Start Engraving
                    {/if}
                  </button>
                </div>
              </div>
            </div>
          </div>
        </Tabs.Content>

        <Tabs.Content value="control" class="flex-1 overflow-hidden mt-0">
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
        </Tabs.Content>
      </Tabs.Root>
    </div>
  </div>

  <StatusMessage {status} />
</main>
