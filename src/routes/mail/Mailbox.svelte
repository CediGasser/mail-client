<script lang="ts">
  import type { Mailbox } from '$lib/types'

  interface Props extends Mailbox {
    onselect?: (mailbox: Mailbox) => void
    selected?: boolean
  }

  let { onselect, selected, ...mailbox }: Props = $props()

  let displayName: string = $derived(mailbox.name.replace('[Gmail]/', ''))
</script>

<button
  class="p-2 border-t border-gray-200 hover:bg-gray-100 cursor-pointer {selected
    ? 'outline outline-2 outline-indigo-400'
    : ''} flex flex-col gap-1"
  onclick={() => {
    onselect?.(mailbox)
  }}
  onkeydown={(e) => {
    if (e.key === 'Enter') {
      onselect?.(mailbox)
    }
  }}
>
  <h2 class="text-lg font-semibold text-start">{displayName}</h2>
  <span class="text-sm text-gray-500">
    {mailbox.attributes.join(', ')}
  </span>
</button>
