import type { Component } from 'svelte'

import Mail from '@lucide/svelte/icons/mail'
import Archive from '@lucide/svelte/icons/archive'
import PencilLine from '@lucide/svelte/icons/pencil-line'
import Send from '@lucide/svelte/icons/send'
import Flag from '@lucide/svelte/icons/flag'
import Trash from '@lucide/svelte/icons/trash'
import ArchiveX from '@lucide/svelte/icons/archive-x'
import CircleAlert from '@lucide/svelte/icons/circle-alert'
import Folder from '@lucide/svelte/icons/folder'

import { archiveMessage, deleteMessage, getEnvelopes } from '$lib/commands'
import { Message } from './message.svelte'
import type { Account } from './account.svelte'

export class Mailbox {
  public account: Account

  public name: string
  public display_name: string
  public delimiter: string
  public attributes: string[] = $state<string[]>([])
  public messages: Message[] = $state<Message[]>([])
  public syncState: 'idle' | 'syncing' | 'error' | 'initial' = $state('initial')

  constructor(
    account: Account,
    name: string,
    display_name: string,
    delimiter: string,
    attributes: string[] = [],
    messages: Message[] = []
  ) {
    this.account = account
    this.name = name
    this.display_name = display_name
    this.delimiter = delimiter
    this.attributes = attributes
    this.messages = messages
  }

  public getMessage(uid: number): Message | undefined {
    return this.messages.find((message) => message.uid === uid)
  }

  public async syncMessages() {
    this.syncState = 'syncing'

    try {
      // Get the messages for the mailbox
      const envelopes = await getEnvelopes(this.account.email, this.name).then(
        (envelopes) => {
          envelopes.sort((a, b) => {
            return b.date.getTime() - a.date.getTime()
          })
          return envelopes
        }
      )

      // Convert envelopes to messages
      this.messages = envelopes.map((envelope) => {
        return new Message(
          this,
          envelope.uid,
          envelope.date,
          envelope.from,
          envelope.to,
          envelope.cc,
          envelope.bcc,
          envelope.subject,
          envelope.headers,
          envelope.flags
        )
      })
    } catch (error) {
      console.error('Failed to sync messages:', error)
      this.syncState = 'error'
      return
    }

    // After syncing, set syncState to 'idle' or 'error' based on the result
    this.syncState = 'idle'
  }

  public async deleteMessage(uid: number): Promise<void> {
    try {
      await deleteMessage(this.account.email, this.name, uid)
      this.messages = this.messages.filter((message) => message.uid !== uid)

      // Sync the trash mailbox if we can find it to reflect the changes
      this.account
        .searchMailboxByAttribute(MailboxAttribute.TRASH)
        ?.syncMessages()
    } catch (error) {
      console.error('Error deleting message:', error)
    }
  }

  public async archiveMessage(uid: number): Promise<void> {
    try {
      await archiveMessage(this.account.email, this.name, uid)
      this.messages = this.messages.filter((message) => message.uid !== uid)

      // Sync the archive mailbox if we can find it to reflect the changes
      this.account
        .searchMailboxByAttribute(MailboxAttribute.ALL)
        ?.syncMessages()
    } catch (error) {
      console.error('Error archiving message:', error)
    }
  }

  get icon(): Component {
    if (this.name === 'INBOX') {
      return Mail
    }
    if (this.attributes.includes(MailboxAttribute.ALL)) {
      return Archive
    }
    if (this.attributes.includes(MailboxAttribute.DRAFTS)) {
      return PencilLine
    }
    if (this.attributes.includes(MailboxAttribute.SENT)) {
      return Send
    }
    if (this.attributes.includes(MailboxAttribute.FLAGGED)) {
      return Flag
    }
    if (this.attributes.includes(MailboxAttribute.TRASH)) {
      return Trash
    }
    if (this.attributes.includes(MailboxAttribute.JUNK)) {
      return ArchiveX
    }
    if (this.attributes.includes(MailboxAttribute.IMPORTANT)) {
      return CircleAlert
    }
    return Folder
  }

  get displayName(): string {
    this.display_name.replace('[Gmail]/', '')

    if (this.display_name === 'INBOX') {
      return 'Inbox'
    }
    if (this.attributes.includes('\\All')) {
      return 'All'
    }
    if (this.attributes.includes('\\Drafts')) {
      return 'Drafts'
    }
    if (this.attributes.includes('\\Sent')) {
      return 'Sent'
    }
    if (this.attributes.includes('\\Flagged')) {
      return 'Flagged'
    }
    if (this.attributes.includes('\\Trash')) {
      return 'Trash'
    }
    if (this.attributes.includes('\\Junk')) {
      return 'Junk'
    }
    if (this.attributes.includes('\\Important')) {
      return 'Important'
    }

    return this.display_name
  }
}

export class MailboxAttribute {
  public static ALL = '\\All' as const
  public static DRAFTS = '\\Drafts' as const
  public static SENT = '\\Sent' as const
  public static FLAGGED = '\\Flagged' as const
  public static TRASH = '\\Trash' as const
  public static JUNK = '\\Junk' as const
  public static IMPORTANT = '\\Important' as const
  public static INBOX = 'INBOX' as const
}
