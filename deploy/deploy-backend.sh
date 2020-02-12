
echo "Building docker image."
# set docker to local
unset DOCKER_TLS_VERIFY
unset DOCKER_HOST
# Build images on local machine, and publish it to $DROPLET_NAME
unset DOCKER_CERT_PATH
unset DOCKER_HOST

# build
case $1 in
     prod)
          echo Building production
          source ~/.env.prod
          ;;
     *)
          echo Building local verison
          source ~/.env.local
          ;;
esac

docker-compose build
docker-compose push


# deploy
if [[ $1 == prod ]]; then
    eval $(docker-machine env $DROPLET_NAME)
    docker-compose pull postgres

    docker pull quay.io/vldm/fetcher
    docker pull quay.io/vldm/stegosd:diet
    docker pull quay.io/vldm/explorer
fi

docker-compose up --no-build -d