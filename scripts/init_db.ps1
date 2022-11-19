# bool variable to launch or not docker
param([bool] $SKIP_DOCKER=0) 

# Modification of the error handling of ps
$oldPreference = $ErrorActionPreference
$ErrorActionPreference = "stop"

# check if psql exist
try {if(Get-Command psql){"psql exists"}} 
Catch {"psql does not exist"}

# check if sqlx exist
try {if(Get-Command sqlx){"sqlx exists"}} 
Catch {"sqlx does not exist. cargo install sqlx-cli --no-default-features --features rustls,postgres"}
Finally {$ErrorActionPreference=$oldPreference}
# set env variable
$DB_PASSWORD = if ($Env:DB_PASSWORD) {$Env:DB_PASSWORD} else {"password"};
$DB_NAME = if ($Env:DB_NAME) {$Env:DB_NAME} else {"newsletter"};
$DB_PORT = if ($Env:DB_PORT) {$Env:DB_PORT} else {"5432"};
$DB_USER = if ($env:DB_USER) {$env:DB_USER} else { "postgres" };
$DB_PASSWORD
$DB_NAME
$DB_PORT
$DB_USER
# do not launch docker if already running
if ( -Not $SKIP_DOCKER)
{
    #docker run instance of postgres
try{
docker run `
--name postgres `
-e POSTGRES_USER=$DB_USER `
-e POSTGRES_PASSWORD=$DB_PASSWORD `
-e POSTGRES_DB=$DB_NAME `
-p $DB_PORT`:5432 `
-d postgres `
postgres -N 1000 } # Assuming you used the default parameters to launch Postgres in Docker!
catch{"Either docker is stopped or it cannot launch postgres."}
# shows the containers running
docker ps
}

# try to open connection on postgres
Do{
    Write-output "Trying to open a psql session on postgres..."
    Start-Sleep -Seconds 3;
    psql -h "localhost" -U $DB_USER -p $DB_PORT -d $DB_NAME -c "\q";
} Until ($lastexitcode -eq '0')
Write-output "DB up and running on port $DB_PORT";

# Migrate the db using sqlx
try{
$env:DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"
$env:DATABASE_URL
sqlx database create
sqlx migrate run
Write-output "Postgres has been migrated, ready to go!"
}
Catch{
    Write-output "Issue while migrating db..."
}
