import type { LayoutLoad } from './$types'
import { getAccount } from '$lib/mail/account.svelte'

export const load: LayoutLoad = async ({ params }) => {
  const email = params.account
  const account = getAccount(email)
  await account.syncMailboxes()

  return {
    email,
  }
}
