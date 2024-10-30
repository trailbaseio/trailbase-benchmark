/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("hhgefcssn6fffv8")

  collection.listRule = ""

  return dao.saveCollection(collection)
}, (db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("hhgefcssn6fffv8")

  collection.listRule = "@request.auth.id = @collection.room_members.user && id = @collection.room_members.room"

  return dao.saveCollection(collection)
})
