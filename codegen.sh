export NODE_TLS_REJECT_UNAUTHORIZED=0
npx @hey-api/openapi-ts \
  -i https://0.0.0.0:8000/openapi.json \
  -o slink_client/src/lib/api \
  -c @hey-api/client-axios