<script>
  import * as Card from '$lib/components/ui/card';
  import { Label } from '$lib/components/ui/label';
  import * as Select from '$lib/components/ui/select';
  import { Slider } from '$lib/components/ui/slider';
  import { Button } from '$lib/components/ui/button';

  function getLayoutLabel(value) {
    const labels = {
      auto: 'Auto (Smart)',
      single: 'Single Line',
      stacked: 'Stacked Names',
    };
    return labels[value] || value;
  }

  let { designSettings = $bindable(), loadPreset } = $props();
</script>

<Card.Root class="border-slate-700 bg-slate-800/50">
  <Card.Header>
    <Card.Title class="flex items-center gap-2 text-white">
      <span class="text-xl">🎨</span>
      Design Settings
    </Card.Title>
    <Card.Description class="text-slate-400">Customize the layout and appearance</Card.Description>
  </Card.Header>
  <Card.Content class="space-y-6">
    <!-- Preset Buttons -->
    <div class="space-y-2">
      <Label class="text-slate-200">Quick Presets</Label>
      <div class="grid grid-cols-2 gap-2">
        <Button
          variant="outline"
          onclick={() => loadPreset('elegant')}
          class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
        >
          <span class="mr-2">✨</span>
          Elegant
        </Button>
        <Button
          variant="outline"
          onclick={() => loadPreset('modern')}
          class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
        >
          <span class="mr-2">🌟</span>
          Modern
        </Button>
        <Button
          variant="outline"
          onclick={() => loadPreset('classic')}
          class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
        >
          <span class="mr-2">📜</span>
          Classic
        </Button>
        <Button
          variant="outline"
          onclick={() => loadPreset('bold')}
          class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
        >
          <span class="mr-2">💪</span>
          Bold
        </Button>
      </div>
    </div>

    <!-- Layout Style -->
    <div class="space-y-2">
      <Label for="layout" class="text-slate-200">Layout Style</Label>
      <Select.Root
        selected={{ value: designSettings.layout, label: getLayoutLabel(designSettings.layout) }}
      >
        <Select.Trigger
          id="layout"
          class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600"
        >
          <Select.Value placeholder="Select layout" />
        </Select.Trigger>
        <Select.Content class="border-slate-600 bg-slate-800">
          <Select.Item value="auto" onclick={() => (designSettings.layout = 'auto')}
            >Auto (Smart)</Select.Item
          >
          <Select.Item value="single" onclick={() => (designSettings.layout = 'single')}
            >Single Line</Select.Item
          >
          <Select.Item value="stacked" onclick={() => (designSettings.layout = 'stacked')}
            >Stacked Names</Select.Item
          >
        </Select.Content>
      </Select.Root>
    </div>

    <!-- Name Size -->
    <div class="space-y-3">
      <div class="flex items-center justify-between">
        <Label for="nameSize" class="text-slate-200">Name Size</Label>
        <span class="text-sm font-semibold text-blue-400">{designSettings.nameSize}mm</span>
      </div>
      <Slider
        id="nameSize"
        min={5}
        max={12}
        step={0.5}
        value={[designSettings.nameSize]}
        onValueChange={(value) => (designSettings.nameSize = value[0])}
        class="[&_[role=slider]]:bg-blue-500 [&_[role=slider]]:border-blue-400"
      />
      <div class="flex justify-between text-xs text-slate-400">
        <span>5mm</span>
        <span>12mm</span>
      </div>
    </div>

    <!-- Date Size -->
    <div class="space-y-3">
      <div class="flex items-center justify-between">
        <Label for="dateSize" class="text-slate-200">Date Size</Label>
        <span class="text-sm font-semibold text-blue-400">{designSettings.dateSize}mm</span>
      </div>
      <Slider
        id="dateSize"
        min={4}
        max={10}
        step={0.5}
        value={[designSettings.dateSize]}
        onValueChange={(value) => (designSettings.dateSize = value[0])}
        class="[&_[role=slider]]:bg-blue-500 [&_[role=slider]]:border-blue-400"
      />
      <div class="flex justify-between text-xs text-slate-400">
        <span>4mm</span>
        <span>10mm</span>
      </div>
    </div>
  </Card.Content>
</Card.Root>
