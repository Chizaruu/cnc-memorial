<script>
  let {
    machineState = $bindable(),
    connectToMachine,
    disconnectMachine,
    homeMachine,
    jogMachine,
    startEngraving,
    pauseEngraving,
    resumeEngraving,
    stopEngraving,
    emergencyStop,
    refreshPorts,
    isValid,
    allSettings,
  } = $props();

  let selectedPort = $state('');
  let jogDistance = $state(1);
  let jogSpeed = $state(500);

  $effect(() => {
    if (machineState.availablePorts.length > 0 && !selectedPort) {
      selectedPort = machineState.availablePorts[0];
    }
  });
</script>

<div class="machine-control">
  <!-- Connection Panel -->
  <div class="control-panel">
    <div class="panel-header">
      <h2>🔌 Machine Connection</h2>
      <div class="connection-status" class:connected={machineState.connected}>
        {machineState.connected ? '🟢 Connected' : '🔴 Disconnected'}
      </div>
    </div>

    <div class="connection-controls">
      {#if !machineState.connected}
        <div class="port-selection">
          <label for="port-select">Serial Port:</label>
          <div class="port-input-group">
            <select id="port-select" bind:value={selectedPort}>
              {#each machineState.availablePorts as port}
                <option value={port}>{port}</option>
              {/each}
            </select>
            <button class="btn-icon" onclick={refreshPorts} title="Refresh Ports"> 🔄 </button>
          </div>
        </div>
        <button
          class="btn btn-success"
          onclick={() => connectToMachine(selectedPort)}
          disabled={!selectedPort}
        >
          📡 Connect to CNC
        </button>
      {:else}
        <div class="connected-info">
          <p><strong>Port:</strong> {machineState.port}</p>
          <p><strong>Status:</strong> {machineState.status}</p>
        </div>
        <button class="btn btn-secondary" onclick={disconnectMachine}> 🔌 Disconnect </button>
      {/if}
    </div>
  </div>

  {#if machineState.connected}
    <!-- Machine Status Panel -->
    <div class="control-panel">
      <div class="panel-header">
        <h2>📊 Machine Status</h2>
        <div class="machine-status" class:alarm={machineState.status === 'Alarm'}>
          {machineState.status}
        </div>
      </div>

      <div class="status-grid">
        <div class="position-display">
          <h3>Machine Position</h3>
          <div class="coordinates">
            <span>X: {machineState.position.x.toFixed(2)}mm</span>
            <span>Y: {machineState.position.y.toFixed(2)}mm</span>
            <span>Z: {machineState.position.z.toFixed(2)}mm</span>
          </div>
        </div>

        <div class="position-display">
          <h3>Work Position</h3>
          <div class="coordinates">
            <span>X: {machineState.workPosition.x.toFixed(2)}mm</span>
            <span>Y: {machineState.workPosition.y.toFixed(2)}mm</span>
            <span>Z: {machineState.workPosition.z.toFixed(2)}mm</span>
          </div>
        </div>
      </div>

      {#if machineState.isRunning}
        <div class="progress-section">
          <h3>Engraving Progress</h3>
          <div class="progress-bar">
            <div class="progress-fill" style="width: {machineState.progress}%"></div>
            <span class="progress-text">{machineState.progress.toFixed(1)}%</span>
          </div>
        </div>
      {/if}
    </div>

    <!-- Control Actions Panel -->
    <div class="control-panel">
      <div class="panel-header">
        <h2>🎮 Machine Control</h2>
      </div>

      <div class="control-actions">
        <!-- Homing -->
        <div class="action-group">
          <button class="btn btn-warning" onclick={homeMachine} disabled={machineState.isRunning}>
            🏠 Home Machine
          </button>
          <small>{machineState.isHomed ? '✅ Homed' : '❌ Not Homed'}</small>
        </div>

        <!-- Job Control -->
        <div class="action-group">
          <div class="job-controls">
            {#if !machineState.isRunning}
              <button
                class="btn btn-danger btn-large"
                onclick={startEngraving}
                disabled={!isValid || !machineState.isHomed}
              >
                🔥 Start Engraving
              </button>
            {:else}
              <div class="running-controls">
                {#if machineState.isPaused}
                  <button class="btn btn-success" onclick={resumeEngraving}> ▶️ Resume </button>
                {:else}
                  <button class="btn btn-warning" onclick={pauseEngraving}> ⏸️ Pause </button>
                {/if}
                <button class="btn btn-danger" onclick={stopEngraving}> ⏹️ Stop </button>
              </div>
            {/if}
          </div>
        </div>

        <!-- Emergency Stop -->
        <div class="action-group">
          <button class="btn btn-emergency" onclick={emergencyStop}> 🚨 EMERGENCY STOP </button>
        </div>
      </div>
    </div>

    <!-- Manual Jog Panel -->
    <div class="control-panel">
      <div class="panel-header">
        <h2>🕹️ Manual Jog</h2>
      </div>

      <div class="jog-controls">
        <div class="jog-settings">
          <div class="jog-input">
            <label for="jog-distance">Distance (mm):</label>
            <select id="jog-distance" bind:value={jogDistance}>
              <option value={0.1}>0.1</option>
              <option value={1}>1</option>
              <option value={10}>10</option>
              <option value={50}>50</option>
            </select>
          </div>

          <div class="jog-input">
            <label for="jog-speed">Speed (mm/min):</label>
            <input
              id="jog-speed"
              type="number"
              bind:value={jogSpeed}
              min="100"
              max="2000"
              step="100"
            />
          </div>
        </div>

        <div class="jog-pad">
          <!-- Z Controls -->
          <div class="z-controls">
            <button
              class="btn btn-jog"
              onclick={() => jogMachine('Z', jogDistance)}
              disabled={machineState.isRunning}
            >
              Z+
            </button>
            <button
              class="btn btn-jog"
              onclick={() => jogMachine('Z', -jogDistance)}
              disabled={machineState.isRunning}
            >
              Z-
            </button>
          </div>

          <!-- XY Controls -->
          <div class="xy-controls">
            <div class="xy-row">
              <div></div>
              <button
                class="btn btn-jog"
                onclick={() => jogMachine('Y', jogDistance)}
                disabled={machineState.isRunning}
              >
                Y+
              </button>
              <div></div>
            </div>
            <div class="xy-row">
              <button
                class="btn btn-jog"
                onclick={() => jogMachine('X', -jogDistance)}
                disabled={machineState.isRunning}
              >
                X-
              </button>
              <button
                class="btn btn-home-xy"
                onclick={() => {
                  jogMachine('X', -machineState.workPosition.x);
                  jogMachine('Y', -machineState.workPosition.y);
                }}
                disabled={machineState.isRunning}
              >
                XY Home
              </button>
              <button
                class="btn btn-jog"
                onclick={() => jogMachine('X', jogDistance)}
                disabled={machineState.isRunning}
              >
                X+
              </button>
            </div>
            <div class="xy-row">
              <div></div>
              <button
                class="btn btn-jog"
                onclick={() => jogMachine('Y', -jogDistance)}
                disabled={machineState.isRunning}
              >
                Y-
              </button>
              <div></div>
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .machine-control {
    width: 100%;
    padding: 20px;
    overflow-y: auto;
    background: linear-gradient(135deg, #f8f9fa, #e9ecef);
  }

  .control-panel {
    background: white;
    border-radius: 12px;
    padding: 25px;
    margin-bottom: 20px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    padding-bottom: 15px;
    border-bottom: 2px solid #ecf0f1;
  }

  .panel-header h2 {
    font-size: 1.3em;
    font-weight: 600;
    color: #2c3e50;
  }

  .connection-status {
    padding: 8px 16px;
    border-radius: 20px;
    font-weight: 600;
    background: #e74c3c;
    color: white;
    transition: all 0.3s ease;
  }

  .connection-status.connected {
    background: #27ae60;
  }

  .connection-controls {
    display: flex;
    flex-direction: column;
    gap: 15px;
  }

  .port-selection {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .port-input-group {
    display: flex;
    gap: 10px;
  }

  .port-input-group select {
    flex: 1;
    padding: 12px;
    border: 2px solid #e9ecef;
    border-radius: 8px;
    font-size: 14px;
  }

  .btn-icon {
    padding: 12px;
    background: #ecf0f1;
    border: 1px solid #bdc3c7;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .btn-icon:hover {
    background: #d5dbdb;
  }

  .connected-info {
    background: #d5f4e6;
    padding: 15px;
    border-radius: 8px;
    border-left: 4px solid #27ae60;
  }

  .machine-status {
    padding: 8px 16px;
    border-radius: 20px;
    font-weight: 600;
    background: #3498db;
    color: white;
  }

  .machine-status.alarm {
    background: #e74c3c;
    animation: pulse 1s infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.7;
    }
  }

  .status-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
    margin-bottom: 20px;
  }

  .position-display {
    background: #f8f9fa;
    padding: 15px;
    border-radius: 8px;
    border-left: 4px solid #3498db;
  }

  .position-display h3 {
    font-size: 1em;
    margin-bottom: 10px;
    color: #2c3e50;
  }

  .coordinates {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .coordinates span {
    font-family: 'Courier New', monospace;
    font-weight: 600;
    background: white;
    padding: 8px;
    border-radius: 4px;
    border: 1px solid #e9ecef;
  }

  .progress-section {
    margin-top: 20px;
  }

  .progress-section h3 {
    margin-bottom: 10px;
    color: #2c3e50;
  }

  .progress-bar {
    position: relative;
    background: #ecf0f1;
    border-radius: 20px;
    height: 30px;
    overflow: hidden;
  }

  .progress-fill {
    background: linear-gradient(90deg, #3498db, #2980b9);
    height: 100%;
    border-radius: 20px;
    transition: width 0.3s ease;
  }

  .progress-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: #2c3e50;
    font-weight: 600;
  }

  .control-actions {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .action-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .job-controls {
    display: flex;
    justify-content: center;
  }

  .running-controls {
    display: flex;
    gap: 15px;
  }

  .btn {
    padding: 12px 20px;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
  }

  .btn-large {
    padding: 18px 30px;
    font-size: 18px;
  }

  .btn-success {
    background: #27ae60;
    color: white;
  }

  .btn-success:hover:not(:disabled) {
    background: #229954;
    transform: translateY(-2px);
  }

  .btn-secondary {
    background: #95a5a6;
    color: white;
  }

  .btn-secondary:hover:not(:disabled) {
    background: #7f8c8d;
    transform: translateY(-2px);
  }

  .btn-warning {
    background: #f39c12;
    color: white;
  }

  .btn-warning:hover:not(:disabled) {
    background: #d68910;
    transform: translateY(-2px);
  }

  .btn-danger {
    background: #e74c3c;
    color: white;
  }

  .btn-danger:hover:not(:disabled) {
    background: #c0392b;
    transform: translateY(-2px);
  }

  .btn-emergency {
    background: linear-gradient(45deg, #e74c3c, #c0392b);
    color: white;
    font-size: 16px;
    padding: 15px 25px;
    border: 3px solid #a93226;
    animation: emergencyGlow 2s infinite;
  }

  @keyframes emergencyGlow {
    0%,
    100% {
      box-shadow: 0 0 5px rgba(231, 76, 60, 0.5);
    }
    50% {
      box-shadow: 0 0 20px rgba(231, 76, 60, 0.8);
    }
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none !important;
  }

  .jog-controls {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .jog-settings {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 15px;
  }

  .jog-input {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .jog-input label {
    font-weight: 500;
    color: #495057;
    font-size: 0.9em;
  }

  .jog-input select,
  .jog-input input {
    padding: 10px;
    border: 2px solid #e9ecef;
    border-radius: 6px;
    font-size: 14px;
  }

  .jog-pad {
    display: grid;
    grid-template-columns: 120px 1fr;
    gap: 20px;
    align-items: center;
  }

  .z-controls {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .xy-controls {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .xy-row {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 8px;
  }

  .btn-jog {
    background: #3498db;
    color: white;
    padding: 12px;
    font-size: 16px;
    font-weight: bold;
  }

  .btn-jog:hover:not(:disabled) {
    background: #2980b9;
  }

  .btn-home-xy {
    background: #e67e22;
    color: white;
    padding: 12px;
    font-size: 12px;
    font-weight: bold;
  }

  .btn-home-xy:hover:not(:disabled) {
    background: #d35400;
  }

  @media (max-width: 768px) {
    .status-grid {
      grid-template-columns: 1fr;
    }

    .jog-settings {
      grid-template-columns: 1fr;
    }

    .jog-pad {
      grid-template-columns: 1fr;
    }

    .running-controls {
      flex-direction: column;
    }
  }
</style>
