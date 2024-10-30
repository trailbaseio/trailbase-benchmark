/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("gm4deo9l30ha9bc")

  collection.indexes = [
    "CREATE INDEX `idx_KXJZyfr` ON `FooBar` (`second`)"
  ]

  return dao.saveCollection(collection)
}, (db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("gm4deo9l30ha9bc")

  collection.indexes = []

  return dao.saveCollection(collection)
})
