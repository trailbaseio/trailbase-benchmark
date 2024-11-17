import type { CollectionConfig } from 'payload'

export const Rooms : CollectionConfig = {
  slug: 'rooms',
  access: {
    create: () => true,
    delete: () => true,
    read: () => true,
    update: () => true,
  },
  fields: [
    {
      name: 'name',
      type: 'text',
      required: true,
    },
  ],
}
