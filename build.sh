#!/usr/bin/env bash

npm i
npx @tailwindcss/cli -i ./style.css -o ./generated.css --minify
