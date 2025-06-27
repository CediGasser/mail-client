<script lang="ts">
  import { Separator } from '$lib/components/ui/separator'
  import type { Message } from '$lib/mail/message.svelte'

  interface Props {
    message: Message
  }
  let { message }: Props = $props()
</script>

<div class="m-2 h-full border rounded-lg shadow-md bg-white">
  <header class="flex flex-col justify-between items-start p-3">
    <div class="flex flex-row justify-between items-center w-full">
      <span class="text-sm text-gray-500">{message.from}</span>
      <span class="text-xs text-gray-500">
        {new Date(message.date).toLocaleString()}
      </span>
    </div>
    <h2 class="text-xl mt-2">
      {message.subject ?? 'No Subject'}
    </h2>
    <div class="flex flex-row items-center gap-2"></div>
  </header>
  <Separator class="mx-4 w-auto" />
  <iframe
    class="w-full h-full"
    title="Email message"
    sandbox="allow-same-origin"
    srcdoc={message.body}
  >
  </iframe>
</div>
