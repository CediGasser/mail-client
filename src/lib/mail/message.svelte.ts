import { getMessage } from '$lib/commands'
import type { Mailbox } from './mailbox.svelte'

export class Message {
  public mailbox: Mailbox

  public uid: number
  public from: string
  public subject: string
  public date: Date
  public read: boolean
  public starred: boolean
  public body?: string
  public syncState: 'idle' | 'syncing' | 'error' | 'initial' = $state('initial')

  constructor(
    mailbox: Mailbox,
    uid: number,
    from: string,
    subject: string,
    date: Date,
    read: boolean = false,
    starred: boolean = false,
    body?: string
  ) {
    this.mailbox = mailbox
    this.uid = uid
    this.from = from
    this.subject = subject
    this.date = date
    this.read = read
    this.starred = starred
    this.body = body
  }

  public async loadMessageBody() {
    if (this.body) {
      return this.body
    }

    this.syncState = 'syncing'

    // loading the message body
    try {
      const message = await getMessage(
        this.mailbox.account.email,
        this.mailbox.name,
        this.uid
      )

      console.log('Loaded message:', message)

      if (message) {
        this.body = message.body || 'No body content available.'
      } else {
        this.body = 'Message not found.'
      }
    } catch (error) {
      this.syncState = 'error'
      console.error('Failed to load message body:', error)
      this.body = 'Failed to load message body.'
    }

    this.syncState = 'idle'
    return this.body
  }
}
