## up: starts all containers in the background without forcing build
up:
	@echo "Starting Docker images..."
	docker compose up -d
	@echo "Docker images started!"

## up_build: stops docker compose (if running), builds all projects and starts docker compose
up_build: build_med_bot
	@echo "Stopping docker images (if running...)"
	docker compose down
	@echo "Building (when required) and starting docker images..."
	docker compose up --build -d
	@echo "Docker images built and started!"

## down: stop docker compose
down:
	@echo "Stopping docker compose..."
	docker compose down
	@echo "Done!"

clean:
	cd ../med-bot-service && cargo clean

## build_med_bot
build_med_bot:
	@echo "Building med-bot binary..."
	cd ../med-bot-service && cargo build --release
	@echo "Done!"
