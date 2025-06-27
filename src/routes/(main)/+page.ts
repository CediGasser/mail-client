import { redirect } from '@sveltejs/kit'
import type { PageLoad } from './$types'
import { getConfig } from '$lib/commands'

export const prerender = true
export const ssr = false

export const load: PageLoad = async () => {
  // Load the app's configuration
  console.log('Loading app configuration...')
  const config = await getConfig()
  console.log('Config:', config)

  // Try to authorize user and redirect to first account's indbox
  if (config.length > 0) {
    const email = config[0].email
    const inboxPath = `/${email}/INBOX/`
    redirect(303, inboxPath)
  }

  // If there are no accounts, redirect to the add-account page
  const addAccountPath = '/add-account'
  redirect(303, addAccountPath)
}
