<script>
  import * as Alert from '$lib/components/ui/alert';
  import { CheckCircle2, XCircle, AlertCircle, Info } from '@lucide/svelte';

  let { status } = $props();

  function getStatusIcon(type) {
    switch (type) {
      case 'success':
        return CheckCircle2;
      case 'error':
        return XCircle;
      case 'warning':
        return AlertCircle;
      default:
        return Info;
    }
  }

  function getAlertVariant(type) {
    switch (type) {
      case 'error':
        return 'destructive';
      default:
        return 'default';
    }
  }

  function getAlertClass(type) {
    switch (type) {
      case 'success':
        return 'border-green-600 bg-green-900/20 text-green-100';
      case 'error':
        return 'border-red-600 bg-red-900/20 text-red-100';
      case 'warning':
        return 'border-yellow-600 bg-yellow-900/20 text-yellow-100';
      default:
        return 'border-blue-600 bg-blue-900/20 text-blue-100';
    }
  }

  function getIconClass(type) {
    switch (type) {
      case 'success':
        return 'text-green-400';
      case 'error':
        return 'text-red-400';
      case 'warning':
        return 'text-yellow-400';
      default:
        return 'text-blue-400';
    }
  }
</script>

{#if status.visible}
  <div
    class="fixed bottom-8 left-1/2 z-50 w-full max-w-md -translate-x-1/2 animate-in slide-in-from-bottom-5 px-4"
  >
    <Alert.Root class="shadow-2xl {getAlertClass(status.type)}">
      <svelte:component
        this={getStatusIcon(status.type)}
        class="h-5 w-5 {getIconClass(status.type)}"
      />
      <Alert.Description class="ml-2 font-medium">
        {status.message}
      </Alert.Description>
    </Alert.Root>
  </div>
{/if}
