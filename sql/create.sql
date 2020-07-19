CREATE TYPE classificacao AS ENUM ('despesa','custo','receita');

CREATE TABLE IF NOT EXISTS erp (
	entry_id int UNIQUE NOT NULL,
	valor int NOT NULL,
	dia int NOT NULL,
	class classificacao NOT NULL,
	origem int,
	destino int,
	PRIMARY KEY(entry_id)
);
