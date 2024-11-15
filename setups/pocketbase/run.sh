#!/bin/sh

# /usr/bin/time ../../vendor/pocketbase-v0.22.21-download --migrationsDir pb_migrations --dir pb_data serve --hooksDir pb_hooks
/usr/bin/time ../../vendor/pocketbase-v0.22.21-cgo-goamd64-v4 --migrationsDir pb_migrations --dir pb_data serve --hooksDir pb_hooks
