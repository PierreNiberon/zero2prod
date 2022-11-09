docker run `
--name postgres `
-e POSTGRES_USER=postgres `
-e POSTGRES_PASSWORD=password `
-e POSTGRES_DB=newsletter `
-p 5432:5432 `
-d postgres `
postgres -N 1000

docker ps