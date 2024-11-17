import type { CollectionConfig } from 'payload'

export const Messages : CollectionConfig = {
  slug: 'messages',
  access: {
    create: async ({ req: { user, payload }, data }) => {
      const id : number | undefined = user?.id ;
      if (!id) {
        return false;
      }

      const room : number | undefined = data?.room;
      if (!room) {
        return false;
      }

      const result = await payload.db.drizzle.run(`SELECT 1 FROM room_members WHERE user_id = ${id} AND room_id = ${room}`)
      if (result.rows.length == 1) {
        return result.rows[0]['1'] === 1;
      }

      return Boolean(false)
    },
    delete: () => true,
    read: () => true,
    update: () => true,
  },
  fields: [
    {
      name: '_owner',
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
    {
      name: 'data',
      type: 'text',
      required: true,
    },
  ],
}
