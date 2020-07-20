CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE classificacao AS ENUM ('despesa','custo','receita');

CREATE TABLE IF NOT EXISTS erp (
	id UUID DEFAULT uuid_generate_v4(),
	valor REAL NOT NULL,
	dia INT NOT NULL,
	class classificacao NOT NULL,
	origem INT,
	destino INT,
	PRIMARY KEY(id)
);
