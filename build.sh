#!/usr/bin/env bash

pnpm i
pnpx @tailwindcss/cli -i ./style.css -o ./generated.css --minify
