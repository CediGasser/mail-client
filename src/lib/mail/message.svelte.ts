import {
  addFlags,
  deleteMessage,
  getMessage,
  removeFlags,
  saveDraft,
  sendEmail,
} from '$lib/commands'
import { MailboxAttribute, type Mailbox } from './mailbox.svelte'
import { debounce } from '$lib/utils'
import type { EmailAddress, Flag } from '$lib/types'

export class Message {
  public mailbox: Mailbox

  public uid: number | undefined
  public date: Date = $state(new Date())
  public from: EmailAddress[] = $state([])
  public to?: EmailAddress[] = $state([])
  public cc?: EmailAddress[] = $state([])
  public bcc?: EmailAddress[] = $state([])
  public subject: string = $state('')
  public headers: Record<string, string> = $state({})
  public flags: Flag[] = $state([])
  public body?: string = $state(undefined)

  public readonly seen: boolean = $derived(this.flags.includes('\\Seen'))
  public readonly flagged: boolean = $derived(this.flags.includes('\\Flagged'))
  public readonly draft: boolean = $derived(this.flags.includes('\\Draft'))

  public syncState: 'idle' | 'syncing' | 'error' | 'initial' = $state('initial')

  public saveDebounced: () => void

  constructor(
    mailbox: Mailbox,
    uid: number,
    date: Date,
    from: EmailAddress[],
    to: EmailAddress[],
    cc: EmailAddress[],
    bcc: EmailAddress[],
    subject: string,
    headers: Record<string, string>,
    flags: Flag[] = [],
    body?: string
  ) {
    this.mailbox = mailbox
    this.uid = uid
    this.date = date
    this.from = from
    this.to = to
    this.cc = cc
    this.bcc = bcc
    this.subject = subject
    this.headers = headers
    this.flags = flags
    this.body = body

    this.saveDebounced = debounce(this.save.bind(this), 1000) // Save draft every second

    if (this.draft) {
      this.loadMessageBody()
    }
  }

  public async toggleFlagged() {
    if (!this.uid) {
      console.warn('Cannot toggle flagged state: UID is not set.')
      return
    }
    if (this.flagged) {
      await removeFlags(
        this.mailbox.account.email,
        this.mailbox.name,
        this.uid,
        '\\Flagged'
      )
      this.flags = this.flags.filter((flag) => flag !== '\\Flagged')
    } else {
      await addFlags(
        this.mailbox.account.email,
        this.mailbox.name,
        this.uid,
        '\\Flagged'
      )
      this.flags.push('\\Flagged')
    }
  }

  public async loadMessageBody() {
    if (!this.uid) {
      console.warn('Cannot load message body: UID is not set.')
      return
    }
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

      this.flags = message.flags || []
      this.body = message.body || 'No body content available.'
    } catch (error) {
      this.syncState = 'error'
      console.error('Failed to load message body:', error)
      this.body = 'Failed to load message body.'
    }

    this.syncState = 'idle'
    return this.body
  }

  public send = async () => {
    console.log('Message details:', {
      from: this.from,
      to: this.to,
      cc: this.cc,
      bcc: this.bcc,
      subject: this.subject,
      body: this.body,
    })

    // Validate that required fields are set
    if (!this.to || !this.subject || !this.body) {
      console.error('Missing required fields: to, subject, or body')
      return
    }

    console.log('Sending message...')
    try {
      await sendEmail(
        this.mailbox.account.email,
        this.to,
        this.cc,
        this.bcc,
        this.subject,
        this.body ?? ''
      )

      if (!this.uid) {
        console.warn(
          'UID is not set, cannot remove draft flags or delete message.'
        )
        return
      }
      // After sending, we remove the draft flag and delete the message
      await removeFlags(
        this.mailbox.account.email,
        this.mailbox.name,
        this.uid,
        '\\Draft'
      )

      await deleteMessage(
        this.mailbox.account.email,
        this.mailbox.name,
        this.uid
      )

      // asyncronously syncronize the draft and sent mailbox
      this.mailbox.syncMessages()
      this.mailbox.account
        .searchMailboxByAttribute(MailboxAttribute.SENT)
        ?.syncMessages()
    } catch (error) {
      console.error('Error sending message:', error)
    }
  }

  static draft = async (mailbox: Mailbox) => {
    const newMessage = new Message(
      mailbox,
      0, // UID will be set when saving the draft
      new Date(),
      [],
      [],
      [],
      [],
      '',
      {},
      ['\\Draft'],
      ''
    )
    newMessage.uid = undefined // UID is not set initially
    const newUid = await newMessage.save()
    mailbox.messages.push(newMessage)
    return newUid
  }

  private save = async () => {
    console.log('Saving draft...')
    try {
      const newUid = await saveDraft(
        this.mailbox.account.email,
        this.mailbox.name,
        this.uid,
        this.subject,
        this.body ?? '',
        this.to,
        this.cc,
        this.bcc
      )

      this.uid = newUid

      return newUid
    } catch (error) {
      console.error('Error saving draft:', error)
    }
  }
}
