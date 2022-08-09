docker exec -i %1 /bin/bash -c "PGPASSWORD=password psql --username user db" < %2
