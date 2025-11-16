<script>
  import * as Card from '$lib/components/ui/card';
  import * as Alert from '$lib/components/ui/alert';
  import { Label } from '$lib/components/ui/label';
  import { Input } from '$lib/components/ui/input';
  import { Slider } from '$lib/components/ui/slider';
  import { AlertCircle } from '@lucide/svelte';

  let { cncSettings = $bindable() } = $props();
</script>

<Card.Root class="border-slate-700 bg-slate-800/50">
  <Card.Header>
    <Card.Title class="flex items-center gap-2 text-white">
      <span class="text-xl">⚙️</span>
      CNC Settings
    </Card.Title>
    <Card.Description class="text-slate-400">Configure machining parameters</Card.Description>
  </Card.Header>
  <Card.Content class="space-y-6">
    <!-- Cut Depth -->
    <div class="space-y-3">
      <div class="flex items-center justify-between">
        <Label for="depth" class="text-slate-200">Cut Depth</Label>
        <span class="text-sm font-semibold text-red-400">{cncSettings.depth.toFixed(2)}mm</span>
      </div>
      <Slider
        id="depth"
        min={0.05}
        max={0.3}
        step={0.01}
        value={[cncSettings.depth]}
        onValueChange={(value) => (cncSettings.depth = value[0])}
        class="[&_[role=slider]]:bg-red-500 [&_[role=slider]]:border-red-400"
      />
      <div class="flex justify-between text-xs text-slate-400">
        <span>0.05mm</span>
        <span>0.30mm</span>
      </div>
      <p class="text-xs text-slate-400">Recommended: 0.12-0.18mm for aluminum</p>
    </div>

    <!-- Feed Rate -->
    <div class="space-y-2">
      <Label for="feedRate" class="text-slate-200">Feed Rate (mm/min)</Label>
      <Input
        id="feedRate"
        type="number"
        min="100"
        max="1000"
        step="10"
        bind:value={cncSettings.feedRate}
        class="border-slate-600 bg-slate-700/50 text-white placeholder:text-slate-400 focus-visible:ring-red-500"
      />
      <p class="text-xs text-slate-400">Speed during cutting</p>
    </div>

    <!-- Plunge Rate -->
    <div class="space-y-2">
      <Label for="plungeRate" class="text-slate-200">Plunge Rate (mm/min)</Label>
      <Input
        id="plungeRate"
        type="number"
        min="50"
        max="500"
        step="10"
        bind:value={cncSettings.plungeRate}
        class="border-slate-600 bg-slate-700/50 text-white placeholder:text-slate-400 focus-visible:ring-red-500"
      />
      <p class="text-xs text-slate-400">Speed when moving into material</p>
    </div>

    <!-- Spindle Speed -->
    <div class="space-y-2">
      <Label for="spindleSpeed" class="text-slate-200">Spindle Speed (RPM)</Label>
      <Input
        id="spindleSpeed"
        type="number"
        min="500"
        max="10000"
        step="100"
        bind:value={cncSettings.spindleSpeed}
        class="border-slate-600 bg-slate-700/50 text-white placeholder:text-slate-400 focus-visible:ring-red-500"
      />
      <p class="text-xs text-slate-400">Recommended: 8000-10000 RPM</p>
    </div>

    <!-- Safety Tips -->
    <Alert.Root class="border-yellow-600 bg-yellow-900/20">
      <AlertCircle class="h-4 w-4 text-yellow-500" />
      <Alert.Title class="text-yellow-200">Safety Tips</Alert.Title>
      <Alert.Description class="text-yellow-100/80">
        <ul class="mt-2 list-inside list-disc space-y-1 text-sm">
          <li>Start with shallow depth (0.10mm)</li>
          <li>Test on scrap material first</li>
          <li>Ensure proper bit is installed</li>
          <li>Secure material firmly</li>
        </ul>
      </Alert.Description>
    </Alert.Root>
  </Card.Content>
</Card.Root>
