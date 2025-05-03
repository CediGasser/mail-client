import { redirect } from '@sveltejs/kit'
import type { PageLoad } from './$types'
import { getConfig, loginWithGoogle } from '$lib/commands'

// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true
export const ssr = false

export const load: PageLoad = async () => {
  // Load the app's configuration
  const config = await getConfig()

  // Try to authorize user and redirect to first account's indbox
  if (config.length > 0) {
    const email = config[0].email
    try {
      await loginWithGoogle(email)
    } catch (error) {
      console.error('Error logging in with Google:', error)
      // Handle the error (e.g., show a notification)
      const somethingWrongPath = '/something-wrong'
      redirect(303, somethingWrongPath)
    }
    const inboxPath = `/${email}/INBOX/`
    redirect(303, inboxPath)
  }

  // If there are no accounts, redirect to the add-account page
  const addAccountPath = '/add-account'
  redirect(303, addAccountPath)
}
