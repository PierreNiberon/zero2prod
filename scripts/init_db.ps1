

# set env variable
$Env:DB_PASSWORD = if ($Env:DB_PASSWORD) {$Env:DB_PASSWORD} else {"password"};
$Env:DB_NAME = if ($Env:DB_NAME) {$Env:DB_NAME} else {"newsletter"};
$Env:DB_PORT = if ($Env:DB_PORT) {$Env:DB_PORT} else {"5432:5432"};
$env:DB_USER = if ($env:DB_USER) {$env:DB_USER} else { "postgres" };
# docker run instance of postgres
docker run `
--name postgres `
-e POSTGRES_USER=$env:DB_USER `
-e POSTGRES_PASSWORD=$env:DB_PASSWORD `
-e POSTGRES_DB=$env:DB_NAME `
-p $Env:DB_PORT `
-d postgres `
postgres -N 1000
# shows the containers running
docker ps