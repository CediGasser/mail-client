<script lang="ts">
  import ScrollArea from '$lib/components/ui/scroll-area/scroll-area.svelte'
  import type { Message } from '$lib/mail/message.svelte'
  import EnvelopeComponent from './envelope.svelte'

  interface Props {
    items: Message[]
    search: string
  }

  let { items, search }: Props = $props()
  let selectedEnvelopeUid: number | null = $state(null)

  let filteredItems = $derived.by(() => {
    return items.filter((envelope) => filterWith(envelope, search))
  })

  function filterWith(envelope: Message, search: string) {
    // Handle special search strings
    if (search === 'unread') {
      return !envelope.seen
    }
    if (search === 'flagged') {
      return envelope.flagged
    }
    if (!search) return true
    const searchLower = search.toLowerCase()
    return (
      envelope.from
        .map((v) => v.address + (v.name ?? ''))
        .join('')
        ?.toLowerCase()
        .includes(searchLower) ||
      envelope.subject?.toLowerCase().includes(searchLower)
    )
  }
</script>

<ScrollArea class="h-full">
  <div class="flex flex-col gap-2 p-2">
    {#each filteredItems as item}
      <EnvelopeComponent
        message={item}
        selected={selectedEnvelopeUid === item.uid}
      />
    {/each}
  </div>
</ScrollArea>
