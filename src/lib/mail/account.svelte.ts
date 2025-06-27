import { SvelteMap } from 'svelte/reactivity'
import { Mailbox } from './mailbox.svelte'
import { getMailboxes } from '$lib/commands'
import { formatMailbox } from '$lib/utils'

const accounts: Map<string, Account> = new Map()

export class Account {
  #mailboxes: Map<string, Mailbox>

  public email: string
  public syncState: 'idle' | 'syncing' | 'error' | 'initial' = $state('initial')

  constructor(email: string) {
    this.#mailboxes = new SvelteMap<string, Mailbox>()
    this.email = email
  }

  public get mailboxes(): Mailbox[] {
    return Array.from(this.#mailboxes.values())
  }

  public getMailbox(name: string): Mailbox | undefined {
    return this.#mailboxes.get(name)
  }

  public async syncMailboxes() {
    this.syncState = 'syncing'

    try {
      const mailboxes = (await getMailboxes(this.email)).map(formatMailbox)

      this.#mailboxes.clear()

      mailboxes.forEach((mailbox) => {
        const newMailbox = new Mailbox(
          this,
          mailbox.name,
          mailbox.display_name,
          mailbox.delimiter,
          mailbox.attributes
        )

        this.#mailboxes.set(mailbox.name, newMailbox)
      })
      this.syncState = 'idle'
    } catch (error) {
      console.error('Failed to sync mailboxes:', error)
      this.syncState = 'error'
    }
  }
}

export function getAccount(email: string): Account {
  if (!accounts.has(email)) {
    accounts.set(email, new Account(email))
  }
  return accounts.get(email)!
}
