dev:
  docker compose up &
  cargo watch -x 'run --'

test:
  cargo test

stress:
  drill --benchmark stress.yml --stats