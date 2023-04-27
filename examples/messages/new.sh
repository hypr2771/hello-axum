curl -v -X PUT http://localhost:3000/topics/644af473f7bc8b2cd7cae876/messages \
     -H 'Content-Type: application/json' \
     -H 'Authorization: Basic dGVzdDp0ZXN0' \
     --data-raw '{"content": "hello"}'