# Sherloque

## What is Sherloque?
Sherloque is a language agnostic SQL SDK generator. Inspired by [GraphqQL Code
Generator](https://www.graphql-code-generator.com/)

## Why Sherloque?
You will want Sherloque if:
1. You want to know if your SQL queries are correct without actually running it
2. You want the return type of your SQL queries
3. You don't want to use macros/template
4. You want to write pure SQL

## How it works?
You just need to provide 3 kind of files:
1. Database schema (CREATE TABLE ...)
2. SQL operations (SELECT/UPDATE/DELETE)
3. Sherloque configuration 

## Example
1. Database Schema
```sql
CREATE TABLE person (
  id    INT           NOT NULL,
  name  VARCHAR(255)  NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE pet (
  id        INT           NOT NULL,
  owner_id  INT           NOT NULL,
  kind      VARCHAR(255)  NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (owner_id) REFERENCES person(id)
);
```
2. Database Operation
(Note that variables are prefixed with `$`)
```sql
-- The name of this file: getUserPetsCount.sql
SELECT person.name, count(*) 
FROM person INNER JOIN pet 
  ON person.id = pet.owner_id
WHERE pet.kind = $petKind
GROUP BY person.id;
```
3. Sherloque config
```json
{
  "language": "typescript",
  "schemas": "./schema/**/*.sql",
  "operations": "./src/**/*.sql",
  "output": "./sdk.ts",
  "database": "mysql"
}
```
4. Result  (generated SDK)
```ts
export default class<T extends Database> {
  constructor(private db: T) {}
  getUserPetsCount(args: {petKind: string}): Promise<{     
    "person.name": string,
    "count(*)": number
  }[]> {
    // generated code
  }
}
```

## How it is different from other libraries?
|Library|SQL Verification|Pure SQL Support|Language Agnostic|Database Connection|
|--|--|--|--|--|
|[Rust Diesel](https://diesel.rs/)|✓|||✓|
|[SQLx](https://github.com/launchbadge/sqlx)|✓|✓||✓|
|Sherloque|✓|✓|✓|

## Why you might not want to use Sherloque
1. It does not support SQL connection, it merely provide SDKs with properly typed functions that generate SQL queries.



