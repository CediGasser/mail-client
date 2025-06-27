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

import { getEnvelopes } from '$lib/commands'
import { Message } from './message.svelte'
import type { Account } from './account.svelte'

export class Mailbox {
  public account: Account

  public name: string
  public display_name: string
  public delimiter: string
  public attributes: string[]
  public messages: Message[]
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
          envelope.from,
          envelope.subject,
          envelope.date,
          envelope.read,
          envelope.starred
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

  get icon(): Component {
    if (this.name === 'INBOX') {
      return Mail
    }
    if (this.attributes.includes('\\All')) {
      return Archive
    }
    if (this.attributes.includes('\\Drafts')) {
      return PencilLine
    }
    if (this.attributes.includes('\\Sent')) {
      return Send
    }
    if (this.attributes.includes('\\Flagged')) {
      return Flag
    }
    if (this.attributes.includes('\\Trash')) {
      return Trash
    }
    if (this.attributes.includes('\\Junk')) {
      return ArchiveX
    }
    if (this.attributes.includes('\\Important')) {
      return CircleAlert
    }
    return Folder
  }
}
