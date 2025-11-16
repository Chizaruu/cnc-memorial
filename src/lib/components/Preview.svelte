<script>
  import * as Card from '$lib/components/ui/card';
  import { Button } from '$lib/components/ui/button';
  import { Grid3x3, ZoomIn, ZoomOut } from '@lucide/svelte';

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
    ctx.fillStyle = '#0f172a';
    ctx.fillRect(0, 0, width, height);

    // Draw plaque outline (300x180mm scaled to canvas)
    const plaqueWidth = 300 * 2;
    const plaqueHeight = 180 * 2;
    const offsetX = (width - plaqueWidth) / 2;
    const offsetY = (height - plaqueHeight) / 2;

    // Plaque background - metallic look
    const gradient = ctx.createLinearGradient(
      offsetX,
      offsetY,
      offsetX + plaqueWidth,
      offsetY + plaqueHeight
    );
    gradient.addColorStop(0, '#cbd5e1');
    gradient.addColorStop(0.5, '#e2e8f0');
    gradient.addColorStop(1, '#cbd5e1');
    ctx.fillStyle = gradient;
    ctx.fillRect(offsetX, offsetY, plaqueWidth, plaqueHeight);

    // Plaque border - darker edges for depth
    ctx.strokeStyle = '#64748b';
    ctx.lineWidth = 3;
    ctx.strokeRect(offsetX, offsetY, plaqueWidth, plaqueHeight);

    // Inner shadow effect
    ctx.strokeStyle = '#94a3b8';
    ctx.lineWidth = 1;
    ctx.strokeRect(offsetX + 5, offsetY + 5, plaqueWidth - 10, plaqueHeight - 10);

    // Grid lines (if enabled)
    if (uiState.showGrid) {
      ctx.strokeStyle = '#94a3b8';
      ctx.lineWidth = 1;
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

    // Draw text - engraved effect
    const layout = calculateOptimalLayout(allSettings);

    layout.lines.forEach((line) => {
      drawText(line.text, line.x, line.y, line.size);
    });
  }

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

  function drawLetter(letter, x, y, size) {
    const letterDef = cncFont[letter];
    if (!letterDef || letterDef.length === 0) return;

    const scaleFactor = (size / 14) * 2;

    // Draw shadow/depth effect first
    ctx.strokeStyle = '#64748b';
    ctx.lineWidth = 3;
    ctx.lineCap = 'round';
    ctx.lineJoin = 'round';

    letterDef.forEach((line) => {
      const [x1, y1, x2, y2] = line;
      const scaledX1 = x + x1 * scaleFactor;
      const scaledY1 = y + y1 * scaleFactor;
      const scaledX2 = x + x2 * scaleFactor;
      const scaledY2 = y + y2 * scaleFactor;

      ctx.beginPath();
      ctx.moveTo(scaledX1 + 1, scaledY1 + 1);
      ctx.lineTo(scaledX2 + 1, scaledY2 + 1);
      ctx.stroke();
    });

    // Draw main letter
    ctx.strokeStyle = '#1e293b';
    ctx.lineWidth = 2.5;

    letterDef.forEach((line) => {
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

<Card.Root class="border-slate-700 bg-slate-800/50 overflow-hidden">
  <Card.Content class="p-6">
    <div class="space-y-4">
      <!-- Canvas -->
      <div class="overflow-hidden rounded-lg border-2 border-slate-700 bg-slate-900 shadow-2xl">
        <canvas use:handleCanvasReady width="800" height="600" class="w-full"></canvas>
      </div>

      <!-- Preview Controls -->
      <div
        class="flex items-center justify-between gap-4 rounded-lg border border-slate-700 bg-slate-800/50 p-4"
      >
        <Button
          variant="outline"
          size="sm"
          onclick={() => (uiState.showGrid = !uiState.showGrid)}
          class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
        >
          <Grid3x3 class="mr-2 h-4 w-4" />
          {uiState.showGrid ? 'Hide Grid' : 'Show Grid'}
        </Button>

        <div class="flex items-center gap-3">
          <Button
            variant="outline"
            size="icon"
            onclick={() => (uiState.zoom = Math.max(0.5, uiState.zoom - 0.1))}
            class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
          >
            <ZoomOut class="h-4 w-4" />
          </Button>
          <span class="min-w-[60px] text-center text-sm font-semibold text-slate-200">
            {(uiState.zoom * 100).toFixed(0)}%
          </span>
          <Button
            variant="outline"
            size="icon"
            onclick={() => (uiState.zoom = Math.min(2.0, uiState.zoom + 0.1))}
            class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
          >
            <ZoomIn class="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>
  </Card.Content>
</Card.Root>
