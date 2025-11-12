#!/bin/bash

# Simple script to run the OompaSale QML application

# Check if qmlscene is available
if command -v qmlscene &> /dev/null; then
    echo "Starting OompaSale POS with qmlscene..."
    qmlscene qml/main.qml
elif command -v qml &> /dev/null; then
    echo "Starting OompaSale POS with qml..."
    qml qml/main.qml
else
    echo "Qt QML runtime not found. Please install Qt6 development tools."
    echo "On Ubuntu: sudo apt install qt6-base-dev qt6-declarative-dev"
    exit 1
fi
