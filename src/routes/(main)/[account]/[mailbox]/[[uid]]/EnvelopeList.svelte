<script lang="ts">
  import ScrollArea from '$lib/components/ui/scroll-area/scroll-area.svelte'
  import type { Envelope, Mailbox } from '$lib/types'
  import EnvelopeComponent from './Envelope.svelte'

  interface Props {
    items: Envelope[]
    account: string
  }

  let { items, account }: Props = $props()
  let selectedEnvelopeUid: number | null = $state(null)
</script>

<ScrollArea class="h-full">
  <div class="flex flex-col gap-2 p-2">
    {#each items as item}
      <EnvelopeComponent
        envelope={item}
        selected={selectedEnvelopeUid === item.uid}
        {account}
      />
    {/each}
  </div>
</ScrollArea>
