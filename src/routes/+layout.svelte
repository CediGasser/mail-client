<script lang="ts">
  import { goto } from '$app/navigation'
  import { event } from '@tauri-apps/api'
  import '../app.css'

  let { children } = $props()

  event.listen('navigate', (event) => {
    const url = event.payload as string
    goto(url)
  })
</script>

{@render children()}
