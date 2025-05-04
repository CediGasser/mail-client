<script lang="ts">
  import { getLinkTo } from '$lib/navigation'
  import type { Envelope } from '$lib/types'
  import Star from '@lucide/svelte/icons/star'
  import { get } from 'svelte/store'

  interface Props extends Envelope {
    onselect?: (envelope: Envelope) => void
    selected?: boolean
  }
  let { onselect, selected, ...envelope }: Props = $props()
</script>

<a
  class="p-2 border-b {envelope.read
    ? 'border-gray-200'
    : 'border-l-4 border-l-blue-500'} {selected
    ? 'outline outline-2 outline-indigo-400'
    : ''} hover:bg-gray-100 cursor-pointer"
  href={getLinkTo(null, null, envelope.uid)}
  onfocus={(e) => {
    e.currentTarget.classList.add('outline-none')
  }}
>
  <div class="flex flex-row items-center justify-between mb-1">
    <span class="text-sm text-gray-500">
      {envelope.from}
    </span>
    <div class="flex flex-row items-center gap-2">
      <span class="text-sm text-gray-500">
        {new Date(envelope.date).toLocaleDateString()}
      </span>
      {#if envelope.starred}
        <Star size="1rem" />
      {:else}
        <Star class="opacity-0" />
      {/if}
    </div>
  </div>
  <h2 class="text-lg font-semibold text-start">{envelope.subject}</h2>
</a>
