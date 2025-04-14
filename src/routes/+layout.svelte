<script lang="ts">
  import { goto } from '$app/navigation'
  import '../app.css'
  import { event } from '@tauri-apps/api'

  let { children } = $props()

  event.listen('navigate', (event) => {
    const url = event.payload as string
    goto(url)
  })
</script>

{@render children()}
