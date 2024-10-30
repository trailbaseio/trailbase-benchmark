/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("ng5nv7k52oysks3")

  collection.createRule = "@request.auth.id = @collection.room_members.user && room = @collection.room_members.room"

  return dao.saveCollection(collection)
}, (db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("ng5nv7k52oysks3")

  collection.createRule = null

  return dao.saveCollection(collection)
})
