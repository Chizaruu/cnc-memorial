<script>
  let { cncSettings = $bindable() } = $props();
</script>

<div class="section">
  <h2>⚙️ CNC Settings</h2>

  <div class="form-group">
    <label for="depth">
      Cut Depth: {cncSettings.depth}mm
    </label>
    <input
      id="depth"
      type="range"
      min="0.05"
      max="0.30"
      step="0.01"
      bind:value={cncSettings.depth}
    />
    <div class="range-labels">
      <span>0.05mm</span>
      <span>0.30mm</span>
    </div>
    <small class="help-text">Recommended: 0.12-0.18mm for aluminum</small>
  </div>

  <div class="form-group">
    <label for="feedRate">Feed Rate (mm/min)</label>
    <input
      id="feedRate"
      type="number"
      min="100"
      max="1000"
      step="10"
      bind:value={cncSettings.feedRate}
    />
    <small class="help-text">Speed during cutting</small>
  </div>

  <div class="form-group">
    <label for="plungeRate">Plunge Rate (mm/min)</label>
    <input
      id="plungeRate"
      type="number"
      min="50"
      max="500"
      step="10"
      bind:value={cncSettings.plungeRate}
    />
    <small class="help-text">Speed when moving into material</small>
  </div>

  <div class="form-group">
    <label for="spindleSpeed">Spindle Speed (RPM)</label>
    <input
      id="spindleSpeed"
      type="number"
      min="500"
      max="10000"
      step="100"
      bind:value={cncSettings.spindleSpeed}
    />
    <small class="help-text">Recommended: 8000-10000 RPM</small>
  </div>

  <div class="info-box">
    <strong>⚠️ Safety Tips:</strong>
    <ul>
      <li>Start with shallow depth (0.10mm)</li>
      <li>Test on scrap material first</li>
      <li>Ensure proper bit is installed</li>
      <li>Secure material firmly</li>
    </ul>
  </div>
</div>

<style>
  .section {
    background: white;
    border-radius: 12px;
    padding: 25px;
    margin-bottom: 20px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  h2 {
    font-size: 1.3em;
    font-weight: 600;
    color: #2c3e50;
    margin-bottom: 20px;
    padding-bottom: 12px;
    border-bottom: 2px solid #ecf0f1;
  }

  .form-group {
    margin-bottom: 18px;
  }

  label {
    display: block;
    margin-bottom: 8px;
    font-weight: 500;
    color: #495057;
    font-size: 0.95em;
  }

  input[type='number'] {
    width: 100%;
    padding: 12px 15px;
    border: 2px solid #e9ecef;
    border-radius: 8px;
    font-size: 15px;
    transition: all 0.3s ease;
  }

  input[type='number']:focus {
    outline: none;
    border-color: #3498db;
    box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
  }

  input[type='range'] {
    width: 100%;
    height: 6px;
    border-radius: 3px;
    background: #e9ecef;
    outline: none;
    -webkit-appearance: none;
    appearance: none;
  }

  input[type='range']::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #e74c3c;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  input[type='range']::-webkit-slider-thumb:hover {
    background: #c0392b;
    transform: scale(1.2);
  }

  input[type='range']::-moz-range-thumb {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #e74c3c;
    cursor: pointer;
    border: none;
    transition: all 0.3s ease;
  }

  input[type='range']::-moz-range-thumb:hover {
    background: #c0392b;
    transform: scale(1.2);
  }

  .range-labels {
    display: flex;
    justify-content: space-between;
    font-size: 0.85em;
    color: #6c757d;
    margin-top: 5px;
  }

  .help-text {
    display: block;
    color: #6c757d;
    font-size: 0.85em;
    margin-top: 5px;
  }

  .info-box {
    background: #fff3cd;
    border-left: 4px solid #ffc107;
    padding: 15px;
    border-radius: 8px;
    margin-top: 20px;
  }

  .info-box strong {
    display: block;
    margin-bottom: 10px;
    color: #856404;
  }

  .info-box ul {
    margin: 0;
    padding-left: 20px;
  }

  .info-box li {
    color: #856404;
    font-size: 0.9em;
    margin-bottom: 5px;
  }
</style>
