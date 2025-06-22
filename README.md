# Simulador de Financiamentos

<https://aloiziomacedo.github.io/house-buying/>

## Explicação Geral

Suponha que você tenha uma dívida de R$500,000.00, e queira
pagar em 25 meses. Caso sobre essa dívida não incorra juros,
você pagaria R$20,000.00 por mês.

Esse processo de pagamento satisfaz duas propriedades:

- O valor que você paga todo mês é constante.
- O valor que você está devendo decresce todo mês por um valor constante.

Note também que essas propriedades são equivalentes. Isto é,
se o valor que você pagar todo mês é o mesmo, então o valor
que você deve decresce constantemente (onde a taxa é igual
ao valor pago por mês), e a recíproca é análoga.

No entanto, na presença de juros, essas propriedades são
contraditórias. Se você pagar um valor constante, o 
valor que você deve não irá descer constantemente, e se o valor
que você deve desce constantemente, você necessariamente não
está pagando um valor constante por mês.

Os dois métodos mais comuns de financiamento, Tabela SAC
e Tabela PRICE, optam cada um por uma das propriedades
em detrimento da outra

## Tabela SAC

A Tabela SAC (_Sistema de Amortização Constante), como o próprio
nome diz, é definida por estar amortizando o valor devido a uma taxa
constante.

Para isso ser realizado, é necessário cancelar o juros todo mês.

Vejamos o nosso caso de R$500,000.00, e vamos supor que o juros é de
1% ao mês:

Estamos querendo reduzir o valor devido a R$480,000.00, que
corresponderia ao pagamento de R$20,000.00 no caso sem juros como
vimos anteriormente.

Incidindo o juros, no entanto, temos que o valor devido começa em 
R$505,000.00. Portanto, nossa primeira parcela tem que ser de 
R$25,000.00, uma vez que, como dito anteriormente, queremos reduzir
o valor devido a R$480,000.00.

Para calcular a segunda parcela, note que o valor devido após incidência
de juros é $484,800.00. Portanto, como agora queremos reduzir o valor devido
a R$460,000.00, temos que a parcela fica R$24,800.00.

Repetindo o raciocínio é fácil calcular o valor de cada parcela. Com
um pouco de atenção, é fácil verificar também que o valor de cada parcela
decai linearmente com taxa igual ao juros aplicado sob a parcela "pura",
sem juros.

## Tabela PRICE

TODO.
