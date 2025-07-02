<script lang="ts">
  import {
    HoverCard,
    HoverCardContent,
    HoverCardTrigger,
  } from '$lib/components/ui/hover-card'
  import { Badge } from '$lib/components/ui/badge'
  import type { EmailAddress } from '$lib/types'
  import X from '@lucide/svelte/icons/x'

  interface Props {
    address: EmailAddress
    onremove?: () => void
  }
  let { address, onremove }: Props = $props()
</script>

<HoverCard>
  <HoverCardTrigger>
    <Badge variant="secondary">
      {#if address?.name}
        {address.name}
      {:else}
        {address?.address}
      {/if}
      {#if onremove}
        <button class="remove-btn" onclick={() => onremove?.()}>
          <X size="1rem" />
        </button>
      {/if}
    </Badge>
  </HoverCardTrigger>
  <HoverCardContent>
    <span>{address?.address}</span>
  </HoverCardContent>
</HoverCard>

<style>
  .remove-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 0.875rem;
    margin-left: 0.25rem;
  }
</style>
