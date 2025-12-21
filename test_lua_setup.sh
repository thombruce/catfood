#!/bin/bash

# Test Lua script with standalone Lua interpreter (if available)
if command -v lua5.4 &> /dev/null; then
    echo "Testing lua_clock.lua with Lua 5.4:"
    lua5.4 examples/lua_clock.lua
elif command -v lua &> /dev/null; then
    echo "Testing lua_clock.lua with Lua:"
    lua examples/lua_clock.lua
else
    echo "Lua interpreter not found, but that's okay for testing the Rust integration"
fi

# Check if files are in place
echo "Checking configuration files:"
if [ -f ~/.config/catfoodBar/config.json ]; then
    echo "✓ config.json exists"
    echo "Content:"
    cat ~/.config/catfoodBar/config.json
else
    echo "✗ config.json not found"
fi

if [ -f ~/.config/catfoodBar/components/lua_clock.lua ]; then
    echo "✓ lua_clock.lua component exists"
else
    echo "✗ lua_clock.lua component not found"
fi