docker exec -i %1 /bin/bash -c "PGPASSWORD=password pg_dump --username user db" > %2