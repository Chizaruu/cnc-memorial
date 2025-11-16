<script>
  import * as Card from '$lib/components/ui/card';
  import { Label } from '$lib/components/ui/label';
  import { Input } from '$lib/components/ui/input';

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

<Card.Root class="border-slate-700 bg-slate-800/50">
  <Card.Header>
    <Card.Title class="flex items-center gap-2 text-white">
      <span class="text-xl">👤</span>
      Personal Information
    </Card.Title>
    <Card.Description class="text-slate-400">
      Enter the memorial information for the plaque
    </Card.Description>
  </Card.Header>
  <Card.Content class="space-y-4">
    <div class="space-y-2">
      <Label for="firstName" class="text-slate-200">First Name</Label>
      <Input
        id="firstName"
        type="text"
        bind:value={personalInfo.firstName}
        placeholder="John"
        class="border-slate-600 bg-slate-700/50 text-white placeholder:text-slate-400 focus-visible:ring-blue-500 border-red-500={!personalInfo.firstName.trim()}"
      />
      {#if !personalInfo.firstName.trim()}
        <p class="text-sm text-red-400">First name is required</p>
      {/if}
    </div>

    <div class="space-y-2">
      <Label for="lastName" class="text-slate-200">Last Name</Label>
      <Input
        id="lastName"
        type="text"
        bind:value={personalInfo.lastName}
        placeholder="Doe"
        class="border-slate-600 bg-slate-700/50 text-white placeholder:text-slate-400 focus-visible:ring-blue-500 border-red-500={!personalInfo.lastName.trim()}"
      />
      {#if !personalInfo.lastName.trim()}
        <p class="text-sm text-red-400">Last name is required</p>
      {/if}
    </div>

    <div class="space-y-2">
      <Label for="birthDate" class="text-slate-200">Birth Date (MM/DD/YYYY)</Label>
      <Input
        id="birthDate"
        type="text"
        bind:value={personalInfo.birthDate}
        placeholder="01/15/1950"
        class="border-slate-600 bg-slate-700/50 text-white placeholder:text-slate-400 focus-visible:ring-blue-500 border-red-500={!validateDate(
          personalInfo.birthDate
        )}"
      />
      {#if !validateDate(personalInfo.birthDate) && personalInfo.birthDate}
        <p class="text-sm text-red-400">Use MM/DD/YYYY format</p>
      {/if}
    </div>

    <div class="space-y-2">
      <Label for="deathDate" class="text-slate-200">Death Date (MM/DD/YYYY)</Label>
      <Input
        id="deathDate"
        type="text"
        bind:value={personalInfo.deathDate}
        placeholder="12/25/2023"
        class="border-slate-600 bg-slate-700/50 text-white placeholder:text-slate-400 focus-visible:ring-blue-500 border-red-500={!validateDate(
          personalInfo.deathDate
        )}"
      />
      {#if !validateDate(personalInfo.deathDate) && personalInfo.deathDate}
        <p class="text-sm text-red-400">Use MM/DD/YYYY format</p>
      {/if}
    </div>
  </Card.Content>
</Card.Root>
