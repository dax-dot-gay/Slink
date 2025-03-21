mkdir -p /slink/certificates/client
mkdir -p /slink/certificates/server

openssl req -x509 -newkey rsa:4096 -sha256 -days 3650 \
  -nodes -keyout /slink/certificates/server/slink.key -out /slink/certificates/server/slink.crt -subj "/CN=server.slink.local"

openssl req -x509 -newkey rsa:4096 -sha256 -days 3650 \
  -nodes -keyout /slink/certificates/client/slink.key -out /slink/certificates/client/slink.crt -subj "/CN=client.slink.local"

chmod -R 777 /slink
chmod -R 777 /workspaces