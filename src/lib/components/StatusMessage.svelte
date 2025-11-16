<script>
  let { status } = $props();

  /**
   * @param {any} type
   */
  function getStatusClass(type) {
    switch (type) {
      case 'success':
        return 'status-success';
      case 'error':
        return 'status-error';
      case 'warning':
        return 'status-warning';
      default:
        return 'status-info';
    }
  }

  /**
   * @param {any} type
   */
  function getStatusIcon(type) {
    switch (type) {
      case 'success':
        return '✅';
      case 'error':
        return '❌';
      case 'warning':
        return '⚠️';
      default:
        return 'ℹ️';
    }
  }
</script>

{#if status.visible}
  <div class="status-toast {getStatusClass(status.type)}">
    <span class="status-icon">{getStatusIcon(status.type)}</span>
    <span class="status-message">{status.message}</span>
  </div>
{/if}

<style>
  .status-toast {
    position: fixed;
    bottom: 30px;
    left: 50%;
    transform: translateX(-50%);
    padding: 16px 24px;
    border-radius: 12px;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
    display: flex;
    align-items: center;
    gap: 12px;
    font-weight: 500;
    z-index: 1000;
    animation: slideUp 0.3s ease-out;
    max-width: 500px;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateX(-50%) translateY(20px);
    }
    to {
      opacity: 1;
      transform: translateX(-50%) translateY(0);
    }
  }

  .status-icon {
    font-size: 1.5em;
  }

  .status-message {
    font-size: 1em;
  }

  .status-success {
    background: #d5f4e6;
    color: #155724;
    border: 2px solid #27ae60;
  }

  .status-error {
    background: #f8d7da;
    color: #721c24;
    border: 2px solid #e74c3c;
  }

  .status-warning {
    background: #fff3cd;
    color: #856404;
    border: 2px solid #ffc107;
  }

  .status-info {
    background: #d1ecf1;
    color: #0c5460;
    border: 2px solid #3498db;
  }
</style>
