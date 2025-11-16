<script>
  let { designSettings = $bindable(), loadPreset } = $props();
</script>

<div class="section">
  <h2>🎨 Design Settings</h2>

  <div class="preset-buttons">
    <button class="preset-btn" onclick={() => loadPreset('elegant')}> ✨ Elegant </button>
    <button class="preset-btn" onclick={() => loadPreset('modern')}> 🌟 Modern </button>
    <button class="preset-btn" onclick={() => loadPreset('classic')}> 📜 Classic </button>
    <button class="preset-btn" onclick={() => loadPreset('bold')}> 💪 Bold </button>
  </div>

  <div class="form-group">
    <label for="layout">Layout Style</label>
    <select id="layout" bind:value={designSettings.layout}>
      <option value="auto">Auto (Smart)</option>
      <option value="single">Single Line</option>
      <option value="stacked">Stacked Names</option>
    </select>
  </div>

  <div class="form-group">
    <label for="nameSize">
      Name Size: {designSettings.nameSize}mm
    </label>
    <input
      id="nameSize"
      type="range"
      min="5"
      max="12"
      step="0.5"
      bind:value={designSettings.nameSize}
    />
    <div class="range-labels">
      <span>5mm</span>
      <span>12mm</span>
    </div>
  </div>

  <div class="form-group">
    <label for="dateSize">
      Date Size: {designSettings.dateSize}mm
    </label>
    <input
      id="dateSize"
      type="range"
      min="4"
      max="10"
      step="0.5"
      bind:value={designSettings.dateSize}
    />
    <div class="range-labels">
      <span>4mm</span>
      <span>10mm</span>
    </div>
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

  .preset-buttons {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
    margin-bottom: 20px;
  }

  .preset-btn {
    padding: 12px;
    background: linear-gradient(135deg, #f8f9fa, #e9ecef);
    border: 2px solid #dee2e6;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .preset-btn:hover {
    background: linear-gradient(135deg, #e9ecef, #dee2e6);
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
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

  select {
    width: 100%;
    padding: 12px 15px;
    border: 2px solid #e9ecef;
    border-radius: 8px;
    font-size: 15px;
    transition: all 0.3s ease;
    background: white;
    cursor: pointer;
  }

  select:focus {
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
    background: #3498db;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  input[type='range']::-webkit-slider-thumb:hover {
    background: #2980b9;
    transform: scale(1.2);
  }

  input[type='range']::-moz-range-thumb {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #3498db;
    cursor: pointer;
    border: none;
    transition: all 0.3s ease;
  }

  input[type='range']::-moz-range-thumb:hover {
    background: #2980b9;
    transform: scale(1.2);
  }

  .range-labels {
    display: flex;
    justify-content: space-between;
    font-size: 0.85em;
    color: #6c757d;
    margin-top: 5px;
  }
</style>
