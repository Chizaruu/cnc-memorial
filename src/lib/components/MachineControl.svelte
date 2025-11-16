<script>
  import * as Card from '$lib/components/ui/card';
  import * as Select from '$lib/components/ui/select';
  import { Button } from '$lib/components/ui/button';
  import { Label } from '$lib/components/ui/label';
  import { Input } from '$lib/components/ui/input';
  import { Badge } from '$lib/components/ui/badge';
  import { Progress } from '$lib/components/ui/progress';
  import {
    Wifi,
    WifiOff,
    RefreshCw,
    Home,
    Play,
    Pause,
    Square,
    AlertTriangle,
    ChevronUp,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    Loader2,
  } from '@lucide/svelte';

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

<div class="h-full overflow-y-auto bg-slate-900/50 p-6">
  <div class="mx-auto max-w-6xl space-y-6">
    <!-- Connection Panel -->
    <Card.Root class="border-slate-700 bg-slate-800/50">
      <Card.Header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            {#if machineState.connected}
              <Wifi class="h-5 w-5 text-green-400" />
            {:else}
              <WifiOff class="h-5 w-5 text-red-400" />
            {/if}
            <Card.Title class="text-white">Machine Connection</Card.Title>
          </div>
          <Badge
            variant={machineState.connected ? 'default' : 'destructive'}
            class={machineState.connected
              ? 'bg-green-600 hover:bg-green-700'
              : 'bg-red-600 hover:bg-red-700'}
          >
            {machineState.connected ? 'Connected' : 'Disconnected'}
          </Badge>
        </div>
      </Card.Header>
      <Card.Content class="space-y-4">
        {#if !machineState.connected}
          <div class="space-y-4">
            <div class="space-y-2">
              <Label for="port-select" class="text-slate-200">Serial Port</Label>
              <div class="flex gap-2">
                <Select.Root
                  selected={{ value: selectedPort, label: selectedPort || 'Select port' }}
                >
                  <Select.Trigger
                    id="port-select"
                    class="flex-1 border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600"
                  >
                    <Select.Value placeholder="Select a port" />
                  </Select.Trigger>
                  <Select.Content class="border-slate-600 bg-slate-800">
                    {#each machineState.availablePorts as port}
                      <Select.Item value={port} onclick={() => (selectedPort = port)}>
                        {port}
                      </Select.Item>
                    {/each}
                  </Select.Content>
                </Select.Root>
                <Button
                  variant="outline"
                  size="icon"
                  onclick={refreshPorts}
                  class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
                >
                  <RefreshCw class="h-4 w-4" />
                </Button>
              </div>
            </div>
            <Button
              class="w-full bg-green-600 hover:bg-green-700"
              onclick={() => connectToMachine(selectedPort)}
              disabled={!selectedPort}
            >
              <Wifi class="mr-2 h-4 w-4" />
              Connect to CNC
            </Button>
          </div>
        {:else}
          <div class="space-y-4">
            <div class="rounded-lg border border-green-600 bg-green-900/20 p-4 space-y-2">
              <div class="flex items-center justify-between text-sm">
                <span class="text-slate-300">Port:</span>
                <span class="font-mono font-semibold text-green-300">{machineState.port}</span>
              </div>
              <div class="flex items-center justify-between text-sm">
                <span class="text-slate-300">Status:</span>
                <Badge variant="outline" class="border-green-600 text-green-300">
                  {machineState.status}
                </Badge>
              </div>
            </div>
            <Button
              variant="outline"
              class="w-full border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
              onclick={disconnectMachine}
            >
              <WifiOff class="mr-2 h-4 w-4" />
              Disconnect
            </Button>
          </div>
        {/if}
      </Card.Content>
    </Card.Root>

    {#if machineState.connected}
      <!-- Machine Status Panel -->
      <Card.Root class="border-slate-700 bg-slate-800/50">
        <Card.Header>
          <div class="flex items-center justify-between">
            <Card.Title class="text-white">Machine Status</Card.Title>
            <Badge
              variant={machineState.status === 'Alarm' ? 'destructive' : 'default'}
              class={machineState.status === 'Alarm' ? 'animate-pulse bg-red-600' : 'bg-blue-600'}
            >
              {machineState.status}
            </Badge>
          </div>
        </Card.Header>
        <Card.Content>
          <div class="grid gap-4 md:grid-cols-2">
            <!-- Machine Position -->
            <div class="rounded-lg border border-slate-600 bg-slate-900/50 p-4">
              <h3 class="mb-3 text-sm font-semibold text-slate-200">Machine Position</h3>
              <div class="space-y-2">
                <div class="flex items-center justify-between rounded bg-slate-800/50 px-3 py-2">
                  <span class="text-sm text-slate-400">X:</span>
                  <span class="font-mono text-sm font-semibold text-white">
                    {machineState.position.x.toFixed(2)}mm
                  </span>
                </div>
                <div class="flex items-center justify-between rounded bg-slate-800/50 px-3 py-2">
                  <span class="text-sm text-slate-400">Y:</span>
                  <span class="font-mono text-sm font-semibold text-white">
                    {machineState.position.y.toFixed(2)}mm
                  </span>
                </div>
                <div class="flex items-center justify-between rounded bg-slate-800/50 px-3 py-2">
                  <span class="text-sm text-slate-400">Z:</span>
                  <span class="font-mono text-sm font-semibold text-white">
                    {machineState.position.z.toFixed(2)}mm
                  </span>
                </div>
              </div>
            </div>

            <!-- Work Position -->
            <div class="rounded-lg border border-slate-600 bg-slate-900/50 p-4">
              <h3 class="mb-3 text-sm font-semibold text-slate-200">Work Position</h3>
              <div class="space-y-2">
                <div class="flex items-center justify-between rounded bg-slate-800/50 px-3 py-2">
                  <span class="text-sm text-slate-400">X:</span>
                  <span class="font-mono text-sm font-semibold text-white">
                    {machineState.workPosition.x.toFixed(2)}mm
                  </span>
                </div>
                <div class="flex items-center justify-between rounded bg-slate-800/50 px-3 py-2">
                  <span class="text-sm text-slate-400">Y:</span>
                  <span class="font-mono text-sm font-semibold text-white">
                    {machineState.workPosition.y.toFixed(2)}mm
                  </span>
                </div>
                <div class="flex items-center justify-between rounded bg-slate-800/50 px-3 py-2">
                  <span class="text-sm text-slate-400">Z:</span>
                  <span class="font-mono text-sm font-semibold text-white">
                    {machineState.workPosition.z.toFixed(2)}mm
                  </span>
                </div>
              </div>
            </div>
          </div>

          {#if machineState.isRunning}
            <div class="mt-4 space-y-2">
              <div class="flex items-center justify-between">
                <h3 class="text-sm font-semibold text-slate-200">Engraving Progress</h3>
                <span class="text-sm font-semibold text-blue-400">
                  {machineState.progress.toFixed(1)}%
                </span>
              </div>
              <Progress value={machineState.progress} class="h-3" />
            </div>
          {/if}
        </Card.Content>
      </Card.Root>

      <!-- Control Actions Panel -->
      <Card.Root class="border-slate-700 bg-slate-800/50">
        <Card.Header>
          <Card.Title class="text-white">Machine Control</Card.Title>
        </Card.Header>
        <Card.Content class="space-y-6">
          <!-- Homing -->
          <div class="space-y-2">
            <Button
              variant="outline"
              class="w-full border-yellow-600 bg-yellow-900/20 text-yellow-200 hover:bg-yellow-900/40 hover:text-yellow-100"
              onclick={homeMachine}
              disabled={machineState.isRunning}
            >
              <Home class="mr-2 h-4 w-4" />
              Home Machine
            </Button>
            <div class="flex items-center justify-center gap-2 text-sm">
              {#if machineState.isHomed}
                <Badge variant="outline" class="border-green-600 text-green-300">✓ Homed</Badge>
              {:else}
                <Badge variant="outline" class="border-slate-600 text-slate-400">✗ Not Homed</Badge>
              {/if}
            </div>
          </div>

          <!-- Job Control -->
          <div class="space-y-3">
            {#if !machineState.isRunning}
              <Button
                class="w-full bg-red-600 text-lg font-semibold hover:bg-red-700 h-14"
                onclick={startEngraving}
                disabled={!isValid || !machineState.isHomed}
              >
                <Play class="mr-2 h-5 w-5" />
                Start Engraving
              </Button>
            {:else}
              <div class="grid grid-cols-2 gap-3">
                {#if machineState.isPaused}
                  <Button class="bg-green-600 hover:bg-green-700" onclick={resumeEngraving}>
                    <Play class="mr-2 h-4 w-4" />
                    Resume
                  </Button>
                {:else}
                  <Button
                    variant="outline"
                    class="border-yellow-600 bg-yellow-900/20 text-yellow-200 hover:bg-yellow-900/40"
                    onclick={pauseEngraving}
                  >
                    <Pause class="mr-2 h-4 w-4" />
                    Pause
                  </Button>
                {/if}
                <Button variant="destructive" onclick={stopEngraving}>
                  <Square class="mr-2 h-4 w-4" />
                  Stop
                </Button>
              </div>
            {/if}
          </div>

          <!-- Emergency Stop -->
          <Button
            variant="destructive"
            class="w-full animate-pulse bg-red-700 text-lg font-bold hover:bg-red-800 h-14 border-2 border-red-500"
            onclick={emergencyStop}
          >
            <AlertTriangle class="mr-2 h-6 w-6" />
            EMERGENCY STOP
          </Button>
        </Card.Content>
      </Card.Root>

      <!-- Manual Jog Panel -->
      <Card.Root class="border-slate-700 bg-slate-800/50">
        <Card.Header>
          <Card.Title class="text-white">Manual Jog Controls</Card.Title>
        </Card.Header>
        <Card.Content class="space-y-6">
          <!-- Jog Settings -->
          <div class="grid gap-4 md:grid-cols-2">
            <div class="space-y-2">
              <Label for="jog-distance" class="text-slate-200">Distance (mm)</Label>
              <Select.Root selected={{ value: jogDistance, label: `${jogDistance}mm` }}>
                <Select.Trigger
                  id="jog-distance"
                  class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600"
                >
                  <Select.Value />
                </Select.Trigger>
                <Select.Content class="border-slate-600 bg-slate-800">
                  <Select.Item value={0.1} onclick={() => (jogDistance = 0.1)}>0.1mm</Select.Item>
                  <Select.Item value={1} onclick={() => (jogDistance = 1)}>1mm</Select.Item>
                  <Select.Item value={10} onclick={() => (jogDistance = 10)}>10mm</Select.Item>
                  <Select.Item value={50} onclick={() => (jogDistance = 50)}>50mm</Select.Item>
                </Select.Content>
              </Select.Root>
            </div>

            <div class="space-y-2">
              <Label for="jog-speed" class="text-slate-200">Speed (mm/min)</Label>
              <Input
                id="jog-speed"
                type="number"
                bind:value={jogSpeed}
                min="100"
                max="2000"
                step="100"
                class="border-slate-600 bg-slate-700/50 text-white placeholder:text-slate-400"
              />
            </div>
          </div>

          <!-- Jog Pad -->
          <div class="grid gap-6 md:grid-cols-[auto_1fr]">
            <!-- Z Controls -->
            <div class="flex flex-col gap-2">
              <Button
                variant="outline"
                size="lg"
                class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white h-16"
                onclick={() => jogMachine('Z', jogDistance)}
                disabled={machineState.isRunning}
              >
                <ChevronUp class="h-6 w-6" />
                <div class="ml-2 text-xs">Z+</div>
              </Button>
              <Button
                variant="outline"
                size="lg"
                class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white h-16"
                onclick={() => jogMachine('Z', -jogDistance)}
                disabled={machineState.isRunning}
              >
                <ChevronDown class="h-6 w-6" />
                <div class="ml-2 text-xs">Z-</div>
              </Button>
            </div>

            <!-- XY Controls -->
            <div class="grid grid-rows-3 gap-2">
              <!-- Row 1 -->
              <div class="grid grid-cols-3 gap-2">
                <div></div>
                <Button
                  variant="outline"
                  size="lg"
                  class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
                  onclick={() => jogMachine('Y', jogDistance)}
                  disabled={machineState.isRunning}
                >
                  <div class="flex flex-col items-center">
                    <ChevronUp class="h-6 w-6" />
                    <span class="text-xs">Y+</span>
                  </div>
                </Button>
                <div></div>
              </div>

              <!-- Row 2 -->
              <div class="grid grid-cols-3 gap-2">
                <Button
                  variant="outline"
                  size="lg"
                  class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
                  onclick={() => jogMachine('X', -jogDistance)}
                  disabled={machineState.isRunning}
                >
                  <div class="flex flex-col items-center">
                    <ChevronLeft class="h-6 w-6" />
                    <span class="text-xs">X-</span>
                  </div>
                </Button>
                <Button
                  variant="outline"
                  size="lg"
                  class="border-orange-600 bg-orange-900/20 text-orange-200 hover:bg-orange-900/40 hover:text-orange-100"
                  onclick={() => {
                    jogMachine('X', -machineState.workPosition.x);
                    jogMachine('Y', -machineState.workPosition.y);
                  }}
                  disabled={machineState.isRunning}
                >
                  <Home class="h-5 w-5" />
                  <div class="ml-1 text-xs">XY</div>
                </Button>
                <Button
                  variant="outline"
                  size="lg"
                  class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
                  onclick={() => jogMachine('X', jogDistance)}
                  disabled={machineState.isRunning}
                >
                  <div class="flex flex-col items-center">
                    <ChevronRight class="h-6 w-6" />
                    <span class="text-xs">X+</span>
                  </div>
                </Button>
              </div>

              <!-- Row 3 -->
              <div class="grid grid-cols-3 gap-2">
                <div></div>
                <Button
                  variant="outline"
                  size="lg"
                  class="border-slate-600 bg-slate-700/50 text-white hover:bg-slate-600 hover:text-white"
                  onclick={() => jogMachine('Y', -jogDistance)}
                  disabled={machineState.isRunning}
                >
                  <div class="flex flex-col items-center">
                    <ChevronDown class="h-6 w-6" />
                    <span class="text-xs">Y-</span>
                  </div>
                </Button>
                <div></div>
              </div>
            </div>
          </div>
        </Card.Content>
      </Card.Root>
    {/if}
  </div>
</div>
