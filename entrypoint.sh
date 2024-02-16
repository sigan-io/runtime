#!/bin/sh

# Start the AWS Runtime API
echo -e "\n INFO Starting the AWS Lambda Runtime API..."

cargo lambda watch \
  --ignore-changes \
  --only-lambda-apis \
  --invoke-address 0.0.0.0 \
  --invoke-port 8080 \
  >/dev/null 2>&1 &

# Watch the runtime for changes and rebuild the project
echo -e "\n INFO Initializing WordPress Runtime..."

exec cargo watch \
  --quiet \
  --exec 'check --color always' \
  --exec 'build --color always' \
  --shell 'cp ${RUNTIME_DEV_DIR}/target/debug/runtime ${RUNTIME_DIR}/bootstrap && \
    echo -e "\n INFO Starting WordPress Runtime...\n" && \
    /var/task/bootstrap'
