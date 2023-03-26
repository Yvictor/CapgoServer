build:
	docker build -f Dockerfile -t capgo .

deploy:
	docker stack deploy -c docker-compose.yml capgo
