<script lang="ts">
  import { invalidate } from '$app/navigation'
  import { markAsFlagged, unmarkAsFlagged } from '$lib/commands'
  import { getLinkTo } from '$lib/navigation'
  import type { Envelope } from '$lib/types'
  import { formatTimeAgo } from '$lib/utils'
  import Star from '@lucide/svelte/icons/star'
  import Dot from '@lucide/svelte/icons/dot'

  interface Props {
    selected?: boolean
    mailbox: string
    envelope: Envelope
  }
  let { selected, mailbox, envelope }: Props = $props()

  const handleToggleStar = async (e: MouseEvent) => {
    e.preventDefault()
    try {
      if (envelope.starred) {
        await unmarkAsFlagged(mailbox, envelope.uid)
      } else {
        await markAsFlagged(mailbox, envelope.uid)
      }
      invalidate('data:envelopes')
    } catch (error) {
      console.error('Error toggling star:', error)
      return
    }
  }
</script>

<a
  class="hover:bg-accent flex flex-col items-start gap-2 rounded-lg border p-3 text-left text-sm transition-all {selected ??
    'bg-muted'}"
  href={getLinkTo(null, null, envelope.uid)}
>
  <div class="flex flex-row w-full items-center justify-between">
    <div class="flex flex-row items-center gap-1">
      {#if !envelope.read}
        <Dot height="1" width="1" size="1rem" strokeWidth="10" color="blue" />
      {/if}
      <span class="text-sm text-gray-500">{envelope.from}</span>
    </div>
    <div class="flex flex-row items-center gap-2">
      <span class="text-xs text-gray-500">
        {formatTimeAgo(envelope.date)}
      </span>
      <div class="star-container {envelope.starred ? 'starred' : ''}">
        <button
          class="p-0 m-0 rounded-sm border-none bg-transparent {envelope.starred
            ? ''
            : 'text-gray-500 hover:text-black'} focus:outline-none"
          onclick={handleToggleStar}
        >
          <Star size="1rem" fill={envelope.starred ? 'black' : 'transparent'} />
        </button>
      </div>
    </div>
  </div>
  <h2 class="text-base {!envelope.read && 'font-semibold'}">
    {envelope.subject}
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
