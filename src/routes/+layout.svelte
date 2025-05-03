<script lang="ts">
  import { goto } from '$app/navigation'
  import { event } from '@tauri-apps/api'
  import '../app.css'

  let { children, data } = $props()

  event.listen('navigate', (event) => {
    const url = event.payload as string
    goto(url)
  })
</script>

<h1>{data.url.toString()}</h1>
<nav><a href="/">root</a> - <a href="/add-account">add-account</a></nav>
{@render children()}
