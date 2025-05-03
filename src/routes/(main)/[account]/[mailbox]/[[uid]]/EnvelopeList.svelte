<script lang="ts">
  import ScrollArea from '$lib/components/ui/scroll-area/scroll-area.svelte'
  import type { Envelope } from '$lib/types'
  import EnvelopeComponent from './Envelope.svelte'

  interface Props {
    items: Envelope[]
    onselect?: (envelope: Envelope) => void
  }

  let { items, onselect }: Props = $props()
  let selectedEnvelopeUid: number | null = $state(null)

  const handleEnvelopeSelected = (envelope: Envelope) => {
    // Handle envelope selection
    selectedEnvelopeUid = envelope.uid
    onselect?.(envelope)
  }
</script>

<ScrollArea class="h-full">
  <div class="flex flex-col">
    {#each items as item}
      <EnvelopeComponent
        {...item}
        onselect={handleEnvelopeSelected}
        selected={selectedEnvelopeUid === item.uid}
      />
    {/each}
  </div>
</ScrollArea>
