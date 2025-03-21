if [ ! -d "./volumes" ]; then
    mkdir -p ./volumes/mongodb
    mkdir -p ./volumes/servers
    mkdir -p ./volumes/certs

    chmod -R 777 ./volumes
fi