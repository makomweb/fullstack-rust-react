APP_URL ?= http://localhost:5173

.PHONY: build up open down dev-up dev-down

start: build up open

build:
	@docker compose -f docker-compose.yml -f docker-compose.dev.yml build
	
up:
	@docker compose -f docker-compose.yml up -d

dev:
	@docker compose -f docker-compose.dev.yml up --build -d

down:
	@docker compose -f docker-compose.yml -f docker-compose.dev.yml down

logs:
	@docker compose -f docker-compose.dev.yml logs --follow

open:
	@if command -v xdg-open > /dev/null 2>&1; then \
		xdg-open $(APP_URL) 2>/dev/null & \
	elif command -v open > /dev/null 2>&1; then \
		open $(APP_URL); \
	elif command -v wslview > /dev/null 2>&1; then \
		wslview $(APP_URL); \
	elif command -v cmd.exe > /dev/null 2>&1; then \
		cmd.exe /c start $(APP_URL); \
	else \
		echo "❌ Could not detect browser launcher."; \
		echo "📍 Please open $(APP_URL) manually"; \
	fi