<script>
  let { allSettings, uiState, cncFont, calculateOptimalLayout } = $props();

  let canvas = $state();
  let ctx = $state();

  export function updateCanvas() {
    if (!canvas || !ctx) return;

    const width = canvas.width;
    const height = canvas.height;

    // Clear canvas
    ctx.clearRect(0, 0, width, height);

    // Draw background
    ctx.fillStyle = '#f8f9fa';
    ctx.fillRect(0, 0, width, height);

    // Draw plaque outline (300x180mm scaled to canvas)
    const plaqueWidth = 300 * 2;
    const plaqueHeight = 180 * 2;
    const offsetX = (width - plaqueWidth) / 2;
    const offsetY = (height - plaqueHeight) / 2;

    // Plaque background
    ctx.fillStyle = '#e0e0e0';
    ctx.fillRect(offsetX, offsetY, plaqueWidth, plaqueHeight);

    // Plaque border
    ctx.strokeStyle = '#999';
    ctx.lineWidth = 2;
    ctx.strokeRect(offsetX, offsetY, plaqueWidth, plaqueHeight);

    // Grid lines (if enabled)
    if (uiState.showGrid) {
      ctx.strokeStyle = '#ccc';
      ctx.lineWidth = 0.5;
      ctx.setLineDash([5, 5]);

      // Vertical center line
      ctx.beginPath();
      ctx.moveTo(offsetX + plaqueWidth / 2, offsetY);
      ctx.lineTo(offsetX + plaqueWidth / 2, offsetY + plaqueHeight);
      ctx.stroke();

      // Horizontal thirds
      for (let i = 1; i < 3; i++) {
        ctx.beginPath();
        ctx.moveTo(offsetX, offsetY + (plaqueHeight / 3) * i);
        ctx.lineTo(offsetX + plaqueWidth, offsetY + (plaqueHeight / 3) * i);
        ctx.stroke();
      }

      ctx.setLineDash([]);
    }

    // Draw text
    const layout = calculateOptimalLayout(allSettings);
    ctx.strokeStyle = '#000';
    ctx.lineWidth = 2;
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';

    layout.lines.forEach((/** @type {{ text: any; x: any; y: any; size: any; }} */ line) => {
      drawText(line.text, line.x, line.y, line.size);
    });
  }

  /**
   * @param {string | any[]} text
   * @param {number} centerX
   * @param {number} y
   * @param {number} fontSize
   */
  function drawText(text, centerX, y, fontSize) {
    const charSpacing = fontSize * 1.2 * 2;
    const totalWidth = (text.length - 1) * charSpacing;
    const startX = centerX - totalWidth / 2;

    for (let i = 0; i < text.length; i++) {
      const char = text[i];
      const charX = startX + i * charSpacing;
      const charY = y * 2;

      drawLetter(char, charX, charY, fontSize);
    }
  }

  /**
   * @param {string | number} letter
   * @param {number} x
   * @param {number} y
   * @param {number} size
   */
  function drawLetter(letter, x, y, size) {
    const letterDef = cncFont[letter];
    if (!letterDef || letterDef.length === 0) return;

    const scaleFactor = (size / 14) * 2;

    letterDef.forEach((/** @type {[any, any, any, any]} */ line) => {
      const [x1, y1, x2, y2] = line;
      const scaledX1 = x + x1 * scaleFactor;
      const scaledY1 = y + y1 * scaleFactor;
      const scaledX2 = x + x2 * scaleFactor;
      const scaledY2 = y + y2 * scaleFactor;

      ctx.beginPath();
      ctx.moveTo(scaledX1, scaledY1);
      ctx.lineTo(scaledX2, scaledY2);
      ctx.stroke();
    });
  }

  /**
   * @param {HTMLCanvasElement} node
   */
  function handleCanvasReady(node) {
    canvas = node;
    ctx = canvas.getContext('2d');
    updateCanvas();
    return {
      destroy() {},
    };
  }

  // Reactive update
  $effect(() => {
    if (canvas && ctx && allSettings) {
      updateCanvas();
    }
  });
</script>

<div class="preview-container">
  <canvas use:handleCanvasReady width="800" height="600"></canvas>

  <div class="preview-controls">
    <button class="control-btn" onclick={() => (uiState.showGrid = !uiState.showGrid)}>
      {uiState.showGrid ? '🔲 Hide Grid' : '⊞ Show Grid'}
    </button>

    <div class="zoom-controls">
      <button
        class="control-btn"
        onclick={() => (uiState.zoom = Math.max(0.5, uiState.zoom - 0.1))}
      >
        🔍−
      </button>
      <span class="zoom-label">{(uiState.zoom * 100).toFixed(0)}%</span>
      <button
        class="control-btn"
        onclick={() => (uiState.zoom = Math.min(2.0, uiState.zoom + 0.1))}
      >
        🔍+
      </button>
    </div>
  </div>
</div>

<style>
  .preview-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: white;
    border-radius: 12px;
    padding: 20px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }

  canvas {
    flex: 1;
    max-width: 100%;
    height: auto;
    border: 2px solid #e9ecef;
    border-radius: 8px;
    background: white;
  }

  .preview-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 15px;
    padding-top: 15px;
    border-top: 2px solid #ecf0f1;
  }

  .control-btn {
    padding: 10px 16px;
    background: #f8f9fa;
    border: 2px solid #dee2e6;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
  }

  .control-btn:hover {
    background: #e9ecef;
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  .zoom-controls {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .zoom-label {
    font-weight: 600;
    color: #495057;
    min-width: 50px;
    text-align: center;
  }
</style>
