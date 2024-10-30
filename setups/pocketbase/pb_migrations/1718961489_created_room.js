/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const collection = new Collection({
    "id": "hhgefcssn6fffv8",
    "created": "2024-06-21 09:18:09.384Z",
    "updated": "2024-06-21 09:18:09.384Z",
    "name": "room",
    "type": "base",
    "system": false,
    "schema": [
      {
        "system": false,
        "id": "nt2rr5zx",
        "name": "name",
        "type": "text",
        "required": false,
        "presentable": false,
        "unique": false,
        "options": {
          "min": null,
          "max": null,
          "pattern": ""
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
  const collection = dao.findCollectionByNameOrId("hhgefcssn6fffv8");

  return dao.deleteCollection(collection);
})
