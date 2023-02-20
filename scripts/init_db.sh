#!/usr/bin/env bash
# sudo docker stop $(sudo docker ps -a -q) -> stop container
# sudo docker rm $(sudo docker ps -a -q)   -> Rm container
# sudo docker ps -a                        -> Show all container
#
set -x
set -eo pipefail

#Faire un check pour psql&&sqlx. Script doit fail si ils ne sont pas installe
if ! [ -x "$(command -v psql)" ];
  then
    echo  >&2 "Error: psql is not installed"
    exit 1
fi

#if ! [ -x "$(command -V sqlx)" ];
#  then
#    echo >&2 "Error sqlx is not installed"
#    echo >&2 "Use : "
#    echo >&2 "cargo install --version='~0.6' sqlx-cli \
#    --no-default-features --features rustls,postgres"
#    echo >&2 "to install it"
#    exit 1
#fi

#Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
#Check if a custom password has been set, otherwise default to 'postgres'
DB_PASSWORD="${POSTGRES_PASSWORD:=postgres}"
#Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
#Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Allow to skip Docker if a dockerized Postgres database is already running
#ToSkip -> SKIP_DOCKER=true ./scripts/init_db.sh
if [[ -z "${SKIP_DOCKER}" ]]
then
#Launch my Postgress using Docker
docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_PORT}":5432 \
  -d postgres \
  postgres -N 1000
  # ^ Increased max number of connection for testing purposes
fi

#Keep pinging Postgres until it's ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q';
do
  >&2 echo "Postgress is still unavailable - sleeping"
  sleep 1
done

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
export DATABASE_URL

##################################################################
#Les commandes SQLX ne fonctionne pas donc faire le reste a la main avec ce suffixe :
#
# --database-url <postgres://postgres:postgres@127.0.0.1:5432/newsletter>
#
#Une fois le script rouler entrer la prochaine commande a la main:
#sudo sqlx migrate add create_subscriptions_table
#sqlx migrate run
#
#Une fois le modele produit dans le migration file
#sqlx database create --database-url postgres://postgres:postgres@127.0.0.1:5432/newsletter
#sqlx migrate run --database-url postgres://postgres:postgres@127.0.0.1:5432/newsletter

