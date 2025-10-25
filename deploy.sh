#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status

echo "🚀 Starting deployment process..."

# Pull latest changes
echo "📦 Pulling latest changes from Git..."
git pull origin main || {
    echo "❌ Git pull failed!"
    exit 1
}

# Build Docker images
echo "🔧 Building Docker images..."
docker compose build
BUILD_EXIT_CODE=$?

# Start containers
echo "🐳 Starting containers..."
docker compose up -d
UP_EXIT_CODE=$?

# Save exit code
EXIT_CODE=$(( BUILD_EXIT_CODE || UP_EXIT_CODE ))

if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Deployment successful!"
else
    echo "⚠️ Deployment failed (exit code: $EXIT_CODE)"
    echo "🧹 Running Docker system prune..."
    docker system prune -af

    echo "🔁 Retrying Docker Compose up..."
    docker compose up -d

    if [ $? -eq 0 ]; then
        echo "✅ Deployment successful after prune!"
    else
        echo "❌ Deployment still failed after retry."
        exit 1
    fi
fi
