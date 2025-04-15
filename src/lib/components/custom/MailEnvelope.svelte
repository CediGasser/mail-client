<script lang="ts">
  interface Props {
    uid: number
    date: String
    subject: String
    from: String
    to: String
    onselect?: (uid: number) => undefined
  }

  let { uid, date, subject, from, to, onselect }: Props = $props()
</script>

<li>
  <button class="w-full" onclick={() => onselect?.(uid)}>
    <div class="p-4 border rounded-lg shadow-md bg-white">
      <div class="text-sm text-gray-500">Date: {date}</div>
      <div class="font-bold text-lg text-gray-800 mt-2">{subject}</div>
      <div class="text-sm text-gray-600 mt-1">From: {from}</div>
      <div class="text-sm text-gray-600 mt-1">To: {to}</div>
    </div>
  </button>
</li>
