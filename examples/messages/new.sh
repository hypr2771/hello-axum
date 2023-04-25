curl -v -X PUT http://localhost:3000/topics/644850113bc7f0ba89e30b3a/messages \
     -H 'Content-Type: application/json' \
     -H 'Authorization: Basic dGVzdDp0ZXN0' \
     --data-raw '{"content": "hello"}'