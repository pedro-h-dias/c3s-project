#/bin/zsh

../c3s-etc/cloud_sql_proxy \
  -instances=sincere-canyon-284001:us-central1:erp-database=tcp:3306 \
  -credential_file=$GOOGLE_APPLICATION_CREDENTIALS
