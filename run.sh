cargo run -p backend
cargo run -p frontend-handlebars
cargo build -p frontend-yew
cd ./frontend-yew && trunk serve
cd ..