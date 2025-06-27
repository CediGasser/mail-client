<script lang="ts">
  import * as Sidebar from '$lib/components/ui/sidebar/index'
  import { getAccount } from '$lib/mail/account.svelte'
  import AppSidebar from './app-sidebar.svelte'

  let { data, children } = $props()

  const account = getAccount(data.email)
</script>

<Sidebar.Provider>
  <AppSidebar {account}></AppSidebar>
  <Sidebar.Inset class="h-dvh">
    {@render children()}
  </Sidebar.Inset>
</Sidebar.Provider>
