/// <reference path="../pb_data/types.d.ts" />
migrate((db) => {
  const collection = new Collection({
    "id": "gl1sfndvhbeizqj",
    "created": "2024-06-21 09:19:06.458Z",
    "updated": "2024-06-21 09:19:06.458Z",
    "name": "room_members",
    "type": "base",
    "system": false,
    "schema": [
      {
        "system": false,
        "id": "sbkvdjop",
        "name": "user",
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
        "id": "my9osfyo",
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
  const collection = dao.findCollectionByNameOrId("gl1sfndvhbeizqj");

  return dao.deleteCollection(collection);
})
