<script>
  let { personalInfo = $bindable() } = $props();

  /**
   * @param {string} dateStr
   */
  function validateDate(dateStr) {
    // Simple date validation (MM/DD/YYYY format)
    const pattern = /^(0[1-9]|1[0-2])\/(0[1-9]|[12][0-9]|3[01])\/\d{4}$/;
    return pattern.test(dateStr);
  }
</script>

<div class="section">
  <h2>👤 Personal Information</h2>

  <div class="form-group">
    <label for="firstName">First Name</label>
    <input
      id="firstName"
      type="text"
      bind:value={personalInfo.firstName}
      placeholder="John"
      class:error={!personalInfo.firstName.trim()}
    />
  </div>

  <div class="form-group">
    <label for="lastName">Last Name</label>
    <input
      id="lastName"
      type="text"
      bind:value={personalInfo.lastName}
      placeholder="Doe"
      class:error={!personalInfo.lastName.trim()}
    />
  </div>

  <div class="form-group">
    <label for="birthDate">Birth Date (MM/DD/YYYY)</label>
    <input
      id="birthDate"
      type="text"
      bind:value={personalInfo.birthDate}
      placeholder="01/15/1950"
      class:error={!validateDate(personalInfo.birthDate)}
    />
    {#if !validateDate(personalInfo.birthDate) && personalInfo.birthDate}
      <small class="error-text">Use MM/DD/YYYY format</small>
    {/if}
  </div>

  <div class="form-group">
    <label for="deathDate">Death Date (MM/DD/YYYY)</label>
    <input
      id="deathDate"
      type="text"
      bind:value={personalInfo.deathDate}
      placeholder="12/25/2023"
      class:error={!validateDate(personalInfo.deathDate)}
    />
    {#if !validateDate(personalInfo.deathDate) && personalInfo.deathDate}
      <small class="error-text">Use MM/DD/YYYY format</small>
    {/if}
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

  input {
    width: 100%;
    padding: 12px 15px;
    border: 2px solid #e9ecef;
    border-radius: 8px;
    font-size: 15px;
    transition: all 0.3s ease;
    font-family: inherit;
  }

  input:focus {
    outline: none;
    border-color: #3498db;
    box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1);
  }

  input.error {
    border-color: #e74c3c;
  }

  input.error:focus {
    box-shadow: 0 0 0 3px rgba(231, 76, 60, 0.1);
  }

  .error-text {
    color: #e74c3c;
    font-size: 0.85em;
    margin-top: 5px;
    display: block;
  }
</style>
