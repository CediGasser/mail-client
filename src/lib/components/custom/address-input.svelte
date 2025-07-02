<script lang="ts">
  import type { EmailAddress } from '$lib/types'
  import EmailAddressBadge from './address-badge.svelte'

  interface Props {
    addresses?: EmailAddress[]
    onchange?: () => void
    placeholder?: string
  }
  let { addresses = $bindable([]), onchange, placeholder }: Props = $props()

  let inputValue = $state('')
  let errorMessage = $state('')

  function addAddress() {
    const trimmedValue = inputValue.trim()
    if (trimmedValue) {
      const [name, address] = trimmedValue.includes('<')
        ? trimmedValue.match(/(.*)<(.*)>/)?.slice(1) || []
        : [null, trimmedValue]

      if (address && validateEmail(address)) {
        addresses = [
          ...addresses,
          { name: name?.trim() || null, address: address.trim() },
        ]
        inputValue = ''
        errorMessage = ''

        onchange?.()
      } else {
        errorMessage = 'Invalid email address'
      }
    }
  }

  function removeAddress(index: number) {
    addresses = addresses.filter((_, i) => i !== index)
    onchange?.()
  }

  function validateEmail(email: string): boolean {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
    return emailRegex.test(email)
  }
</script>

<div class="email-input w-full">
  {#each addresses as address, index}
    <div class="badge-wrapper">
      <EmailAddressBadge {address} onremove={() => removeAddress(index)} />
    </div>
  {/each}
  <input
    class="px-1 bg-transparent w-full"
    type="text"
    bind:value={inputValue}
    onkeydown={(e) => e.key === 'Enter' && addAddress()}
    placeholder={placeholder || 'Add email address'}
  />
  {#if errorMessage}
    <div class="error-message">{errorMessage}</div>
  {/if}
</div>

<style>
  .email-input {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
  }

  .badge-wrapper {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .error-message {
    color: red;
    font-size: 0.875rem;
    margin-top: 0.25rem;
  }
</style>
