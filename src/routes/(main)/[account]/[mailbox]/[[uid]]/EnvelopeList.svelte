<script lang="ts">
  import ScrollArea from '$lib/components/ui/scroll-area/scroll-area.svelte'
  import type { Envelope } from '$lib/types'
  import EnvelopeComponent from './Envelope.svelte'

  interface Props {
    items: Envelope[]
    account: string
    search: string
  }

  let { items, account, search }: Props = $props()
  let selectedEnvelopeUid: number | null = $state(null)

  let filteredItems = $derived.by(() => {
    return items.filter((envelope) => filterWith(envelope, search))
  })

  function filterWith(envelope: Envelope, search: string) {
    // Handle special search strings
    if (search === 'unread') {
      return !envelope.read
    }
    if (search === 'flagged') {
      return envelope.starred
    }
    if (!search) return true
    const searchLower = search.toLowerCase()
    return (
      envelope.from?.toLowerCase().includes(searchLower) ||
      envelope.subject?.toLowerCase().includes(searchLower)
    )
  }
</script>

<ScrollArea class="h-full">
  <div class="flex flex-col gap-2 p-2">
    {#each filteredItems as item}
      <EnvelopeComponent
        envelope={item}
        selected={selectedEnvelopeUid === item.uid}
        {account}
      />
    {/each}
  </div>
</ScrollArea>
