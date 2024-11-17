# Payload CMS Benchark Setup

Built from the starter template, that's why there's so much extra stuff I
didn't bother to remove. Interestingly, the blank template doesn't seem to have
the admin dash setup.

Brigs up tables:

- messages
- rooms
- room members

TODO:

- Automatically bootstrap new DBs by: 
  - Inserting ("room0") into rooms
  - And (user, rooms) values (1, 1) into room_members.
