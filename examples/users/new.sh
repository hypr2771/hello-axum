curl -v -X PUT http://localhost:3000/users \
     -H 'Content-Type: application/json' \
     --data-raw '{"username": "test", "password": "test"}'