TAG := $(shell git rev-list HEAD --max-count=1 --abbrev-commit)
PROJECT-ID := sincere-canyon-284001
INSTANCE-CONNECTION-NAME := sincere-canyon-284001:us-central1:erp-database
IMAGE := erp-service

build:
	docker build . --tag gcr.io/$(PROJECT-ID)/$(IMAGE):$(TAG)

push: build
	docker push gcr.io/$(PROJECT-ID)/$(IMAGE):$(TAG)

deploy: push
	gcloud run deploy \
		--image gcr.io/$(PROJECT-ID)/$(IMAGE):$(TAG) \
		--add-cloudsql-instances $(INSTANCE-CONNECTION-NAME) \
		--update-env-vars INSTANCE_CONNECTION_NAME=$(INSTANCE-CONNECTION-NAME) \
		--update-env-vars GOOGLE_APPLICATION_CREDENTIALS=gcloud.json \
		--update-env-vars DB_USER=$(DB_USER) \
		--update-env-vars DB_PASS=$(DB_PASS) \
		--update-env-vars DB_NAME=$(DB_NAME) \
		--platform managed
