export type Envelope = {
  uid: number
  from: string
  subject: string
  date: string
  read?: boolean
  starred?: boolean
}

export type Address = {
  name: string
  email: string
}
