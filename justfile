# Run app
run:
    cargo watch -x 'run' -w src

# build tailwind css classes in watch mode 
tailwindcss:
    cd node && npx tailwindcss -i input.css -o ../static/css/tailwind.css --watch && cd ..
        
# tailwindcss build
build-tailwindcss:
    cd node && npx tailwindcss -i input.css -o ../static/css/tailwind.css && cd ..

# Build app
build: build-tailwindcss
    cargo build --release

# Preview app
preview: build
    ./target/release/johnethel-school

#Deploy to shuttle
deploy: build-tailwindcss
    fly deploy 
