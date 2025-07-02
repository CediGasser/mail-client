<script lang="ts">
  import { Button } from '$lib/components/ui/button'
  import Separator from '$lib/components/ui/separator/separator.svelte'
  import AddressInput from '$lib/components/custom/address-input.svelte'
  import type { Message } from '$lib/mail/message.svelte'

  interface Props {
    message: Message
  }
  let { message }: Props = $props()
</script>

<div class="m-2 h-full border rounded-lg shadow-md bg-white">
  <header class="flex flex-col justify-between items-start p-3">
    <div class="flex flex-row gap-2 w-full">
      <label for="to">To:</label>
      <AddressInput
        onchange={message.saveDebounced}
        bind:addresses={message.to}
        placeholder="To email"
      />
    </div>
    <div class="flex flex-row gap-2 w-full">
      <label for="cc">CC:</label>
      <AddressInput
        onchange={message.saveDebounced}
        bind:addresses={message.cc}
        placeholder="CC email"
      />
    </div>
    <div class="flex flex-row gap-2 w-full">
      <label for="bcc">BCC:</label>
      <AddressInput
        onchange={message.saveDebounced}
        bind:addresses={message.bcc}
        placeholder="BCC email"
      />
    </div>
    <div class="flex flex-row gap-2 mt-2 w-full">
      <label for="subject">Subject:</label>
      <input
        onchange={message.saveDebounced}
        id="subject"
        type="text"
        bind:value={message.subject}
        class="px-1 bg-transparent w-full"
        placeholder="Email subject"
      />
    </div>
  </header>
  <Separator class="mx-4 w-auto" />
  <textarea
    onchange={message.saveDebounced}
    bind:value={message.body}
    class="border-none p-2 h-64 w-full"
    placeholder="Email body"
  ></textarea>
  <div class="flex flex-row gap-2 m-3">
    <Button variant="outline" class="flex-1" onclick={message.send}>Send</Button
    >
  </div>
</div>
