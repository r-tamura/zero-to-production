#!/usr/bin/env bash
# Usage: ./clean_db.sh

set -e

export PGPASSWORD=password

drop_sqls=$(psql -h 127.0.0.1 -U postgres postgres \
-t -c "select 'drop database \"'||datname||'\";' from pg_database where datistemplate=false and datname like '%-%'";)

echo "${drop_sqls}" |  psql -h 127.0.0.1 -U postgres newsletter
echo 'Dropped all test databases'