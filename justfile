# Default variable
default := ''

# Run app
run external=default port=default port-number=default:
    cd frontend && astro build && cd .. && concurrently "RUSTRO_DEV=true cargo watch -x 'shuttle run   '" "cd frontend && astro dev" -n Rust,Astro

# Build app
build external=default port=default port-number=default:
    cd frontend && astro build && cd .. && cargo build --release

# Preview app
preview external=default port=default port-number=default:
    cd frontend && astro build && cd .. && cargo watch -x 'shuttle run   '

#Deploy to shuttle
deploy:
    cd frontend && astro build && cd .. && cargo shuttle deploy 

# install
install:
    cd frontend && npm install && cd ..
