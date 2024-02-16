#!/bin/sh

# Start the AWS Runtime API
echo -e "\n \e[32mINFO\e[0m Starting the AWS Lambda Runtime API..."

cargo lambda watch \
  --ignore-changes \
  --only-lambda-apis \
  --invoke-address 0.0.0.0 \
  --invoke-port 8080 \
  >/dev/null 2>&1 &

# Watch the runtime for changes and rebuild the project
echo -e "\n \e[32mINFO\e[0m Initializing watcher..."

exec cargo watch \
  --quiet \
  --shell  \
    'echo -e "\n \e[32mINFO\e[0m Building runtime...\n" && \
    cargo check --color always && \
    cargo build --color always && \
    cp ${RUNTIME_DEV_DIR}/target/debug/runtime ${RUNTIME_DIR}/bootstrap && \
    echo -e "\n \e[32mINFO\e[0m Starting runtime...\n" && \
    /var/task/bootstrap'