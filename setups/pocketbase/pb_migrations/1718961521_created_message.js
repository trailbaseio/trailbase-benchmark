/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const collection = new Collection({
    "id": "ng5nv7k52oysks3",
    "created": "2024-06-21 09:18:41.618Z",
    "updated": "2024-06-21 09:18:41.618Z",
    "name": "message",
    "type": "base",
    "system": false,
    "schema": [
      {
        "system": false,
        "id": "3t1wa2na",
        "name": "owner",
        "type": "relation",
        "required": false,
        "presentable": false,
        "unique": false,
        "options": {
          "collectionId": "_pb_users_auth_",
          "cascadeDelete": false,
          "minSelect": null,
          "maxSelect": 1,
          "displayFields": null
        }
      },
      {
        "system": false,
        "id": "fmkwocli",
        "name": "room",
        "type": "relation",
        "required": false,
        "presentable": false,
        "unique": false,
        "options": {
          "collectionId": "hhgefcssn6fffv8",
          "cascadeDelete": false,
          "minSelect": null,
          "maxSelect": 1,
          "displayFields": null
        }
      },
      {
        "system": false,
        "id": "rqaodlzs",
        "name": "data",
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
  const collection = dao.findCollectionByNameOrId("ng5nv7k52oysks3");

  return dao.deleteCollection(collection);
})
