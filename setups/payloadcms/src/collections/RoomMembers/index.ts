import type { CollectionConfig } from 'payload'

export const RoomMembers : CollectionConfig = {
  slug: 'room_members',
  access: {
    create: () => true,
    delete: () => true,
    read: () => true,
    update: () => true,
  },
  fields: [
    {
      name: 'user',
      type: 'relationship',
      relationTo: 'users',
      required: true,
    },
    {
      name: 'room',
      type: 'relationship',
      relationTo: 'rooms',
      required: true,
    },
  ],
}
