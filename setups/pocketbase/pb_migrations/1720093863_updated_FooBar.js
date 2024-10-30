/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("gm4deo9l30ha9bc")

  // remove
  collection.schema.removeField("3rmuebog")

  return dao.saveCollection(collection)
}, (db) => {
  const dao = new Dao(db)
  const collection = dao.findCollectionByNameOrId("gm4deo9l30ha9bc")

  // add
  collection.schema.addField(new SchemaField({
    "system": false,
    "id": "3rmuebog",
    "name": "third",
    "type": "text",
    "required": false,
    "presentable": false,
    "unique": false,
    "options": {
      "min": null,
      "max": null,
      "pattern": ""
    }
  }))

  return dao.saveCollection(collection)
})
