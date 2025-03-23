cd slink_server
cargo watch -x 'run' &
cd ../slink_client
pnpm install
pnpm dev --host 0.0.0.0 --port 5173 --no-clear-screen &
cd ..
sleep infinity