export type Mailbox = {
  name: string
  display_name: string
  delimiter: string
  attributes: string[]
}

export type Envelope = {
  uid: number
  from: string
  subject: string
  date: Date
  read?: boolean
  starred?: boolean
}

export type Address = {
  name: string
  email: string
}

export type Account = {
  email: string
}

export type AccountConfig = Account[]
