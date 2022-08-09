docker exec -i %1 /bin/bash -c "PGPASSWORD=password psql --username user db -c \"drop owned by user;\""
