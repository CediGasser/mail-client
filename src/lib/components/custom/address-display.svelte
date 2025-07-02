<script lang="ts">
  import type { EmailAddress } from '$lib/types'
  import EmailAddressBadge from './address-badge.svelte'

  interface Props {
    addresses?: EmailAddress[]
  }
  let { addresses = $bindable([]) }: Props = $props()
</script>

<div class="wrapper w-full">
  {#each addresses as address}
    <div class="badge-wrapper">
      <EmailAddressBadge {address} />
    </div>
  {/each}
</div>

<style>
  .wrapper {
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
</style>
