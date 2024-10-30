/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const collection = new Collection({
    "id": "gm4deo9l30ha9bc",
    "created": "2024-07-04 11:50:18.197Z",
    "updated": "2024-07-04 11:50:18.197Z",
    "name": "FooBar",
    "type": "base",
    "system": false,
    "schema": [
      {
        "system": false,
        "id": "ibhuce5v",
        "name": "first",
        "type": "text",
        "required": false,
        "presentable": false,
        "unique": false,
        "options": {
          "min": null,
          "max": null,
          "pattern": ""
        }
      },
      {
        "system": false,
        "id": "kpx7qza1",
        "name": "second",
        "type": "text",
        "required": false,
        "presentable": false,
        "unique": false,
        "options": {
          "min": null,
          "max": null,
          "pattern": ""
        }
      },
      {
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
      },
      {
        "system": false,
        "id": "xzcdobtc",
        "name": "number",
        "type": "number",
        "required": false,
        "presentable": false,
        "unique": false,
        "options": {
          "min": null,
          "max": null,
          "noDecimal": false
        }
      },
      {
        "system": false,
        "id": "kz6j1ulx",
        "name": "json",
        "type": "json",
        "required": false,
        "presentable": false,
        "unique": false,
        "options": {
          "maxSize": 2000000
        }
      }
    ],
    "indexes": [],
    "listRule": null,
    "viewRule": null,
    "createRule": null,
    "updateRule": null,
    "deleteRule": null,
    "options": {}
  });

  return Dao(db).saveCollection(collection);
}, (db) => {
  const dao = new Dao(db);
  const collection = dao.findCollectionByNameOrId("gm4deo9l30ha9bc");

  return dao.deleteCollection(collection);
})
