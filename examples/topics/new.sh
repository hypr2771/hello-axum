curl -v -X PUT http://localhost:3000/topics \
     -H 'Content-Type: application/json' \
     -H 'Authorization: Basic dGVzdDp0ZXN0' \
     --data-raw '{"title": "hello"}'