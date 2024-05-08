# Run app
run:
    cd frontend && astro build && cd .. && concurrently "RUSTRO_DEV=true cargo watch -x 'run'" "cd frontend && astro dev" -n Rust,Astro

# Build app
build:
    cd frontend && astro build && cd .. && cargo build --release

# Preview app
preview:
    cd frontend && astro build && cd .. && cargo watch -x 'run'

#Deploy to shuttle
deploy:
    cd frontend && astro build && cd .. && fly deploy 

# install
install:
    cd frontend && npm install && cd ..
