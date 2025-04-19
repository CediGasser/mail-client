<script lang="ts">
  interface Props {
    items: string[]
    onselect?: (mailbox: string) => void
  }

  let { items, onselect }: Props = $props()
</script>

<main class="h-full">
  <div class="flex flex-col">
    {#each items as item}
      <button
        class="p-2 border-t border-gray-200 hover:bg-gray-100 cursor-pointer"
        onclick={() => {
          onselect?.(item)
        }}
        onkeydown={(e) => {
          if (e.key === 'Enter') {
            onselect?.(item)
          }
        }}
        onfocus={(e) => {
          e.currentTarget.classList.add('outline-none')
        }}
      >
        <h2 class="text-lg font-semibold text-start">{item}</h2>
      </button>
    {/each}
  </div>
</main>
