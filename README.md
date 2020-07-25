# ERP Service
## Enterprise Resource Planning

Projeto simples simulando uma ERP. 
Feito em rust, com o Rocket.rs como webframework.
O deploy é feito na GCP, rodando em uma instância de Cloud Run. 

### Rotas implementadas

* GET /lancamento/                  Retorna todos lançamentos do último mês.

* GET /lancamento/valor?value=      Retorna os lançamentos com um dado valor (float32).

* GET /lancamento/dia?value=        Retorna os lançamentos em um dado dia (entre 1 e 30).

* GET /lancamento/origem?value=     Retorna os lançamentos com uma dada origem (entre 1 e 10).

* GET /lancamento/destino?value=    Retorna os lançamentos com um dado destino (entre 1 e 10).

* GET /relatorio/?<dia>&<periodo>   Fornece um relatorio de fluxo de caixa para o dia e periodo fornecidos.

* PUT /lancamento/delete?id=        Deleta um lançamento dado seu identificador UUID.

* POST /lancamento/                 Insere um lançamento novo.

### Input

O input para um novo lançamento é esperado no formato JSON, conforme o modelo a seguir.

{
   "valor": 13.37,
   "dia": 25,
   "class": "Receita",
   "origem": 2
}
