#!/bin/sh

/usr/bin/time ../../vendor/pocketbase/main --migrationsDir pb_migrations --dir pb_data serve --hooksDir pb_hooks
