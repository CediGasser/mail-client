<script lang="ts">
  import * as Sidebar from '$lib/components/ui/sidebar/index'
  import AppSidebar from './app-sidebar.svelte'
  let { data, children } = $props()

  let { mailboxes, account } = data
</script>

<Sidebar.Provider>
  <AppSidebar {mailboxes} {account}></AppSidebar>
  <Sidebar.Inset class="h-dvh">
    {@render children()}
  </Sidebar.Inset>
</Sidebar.Provider>
