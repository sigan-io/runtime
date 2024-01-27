install:
  composer install --working-dir=wp-runtime-prod --optimize-autoloader --no-dev

zip:
  mkdir -p dist
  zip dist/runtime.zip wp-runtime-prod -rx "*/composer.json" "*/composer.lock" "*/.gitignore" "*/serverless.yml" "*/README.md" "*/CHANGELOG.md" "*/LICENSE" "*/.github/*"

bundle:
  just install
  just zip

upload:
  aws s3 cp dist/runtime.zip s3://sigan-runtime-code/runtime.zip

clean:
  rm -rf wp-runtime-prod/vendor
  rm -rf dist
