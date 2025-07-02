<script lang="ts">
  import { getLinkTo } from '$lib/navigation'
  import { formatTimeAgo } from '$lib/utils'
  import Star from '@lucide/svelte/icons/star'
  import Dot from '@lucide/svelte/icons/dot'
  import type { Message } from '$lib/mail/message.svelte'
  import AddressDisplay from '$lib/components/custom/address-display.svelte'

  interface Props {
    selected?: boolean
    message: Message
  }

  let { selected, message }: Props = $props()

  let starred = $derived(message.flagged)

  const handleToggleFlagged = async (e: Event) => {
    e.preventDefault()
    await message.toggleFlagged()
  }
</script>

<a
  class="hover:bg-accent flex flex-col items-start gap-2 rounded-lg border p-3 text-left text-sm transition-all {selected ??
    'bg-muted'}"
  href={getLinkTo(null, null, message.uid ?? null)}
>
  <div class="flex flex-row w-full items-center justify-between">
    <div class="flex flex-row items-start gap-1">
      {#if !message.seen}
        <Dot height="1" width="1" size="1rem" strokeWidth="10" color="blue" />
      {/if}
      {#if message.draft}
        <AddressDisplay addresses={message.to} />
      {:else}
        <AddressDisplay addresses={message.from} />
      {/if}
    </div>
    <div class="flex flex-row items-center gap-2">
      <span class="text-xs text-gray-500">
        {formatTimeAgo(message.date)}
      </span>
      <div class="star-container {starred ? 'starred' : ''}">
        <button
          class="p-0 m-0 rounded-sm border-none bg-transparent {starred
            ? ''
            : 'text-gray-500 hover:text-black'} focus:outline-none"
          onclick={handleToggleFlagged}
        >
          <Star size="1rem" fill={starred ? 'black' : 'transparent'} />
        </button>
      </div>
    </div>
  </div>
  <h2 class="text-base {!message.seen && 'font-semibold'}">
    {message.subject}
  </h2>
</a>

<style>
  .star-container:not(.starred) {
    opacity: 0;
    transition: opacity 0.2s ease-in-out;
  }

  a:hover .star-container {
    opacity: 1;
  }
</style>
