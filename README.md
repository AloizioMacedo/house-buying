# Simulador de Financiamentos

[Simulador](https://aloiziomacedo.github.io/house-buying/) de impacto
financeiro de um financiamento.

Projeta impacto nas finanças pessoais, considerando coisas como rendimento,
bônus anuais, gastos mensais, inflação, etc.

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
em detrimento da outra.

## Tabela SAC

A Tabela SAC (_Sistema de Amortização Constante_), como o próprio
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
de juros é R$484,800.00. Portanto, como agora queremos reduzir o valor devido
a R$460,000.00, temos que a parcela fica R$24,800.00.

Repetindo o raciocínio é fácil calcular o valor de cada parcela. Com
um pouco de atenção, é fácil verificar também que o valor de cada parcela
decai linearmente com taxa igual ao juros aplicado sobre a parcela "pura",
sem juros.

De fato, seja $f$ o valor do financiamento, $n$ o número de parcelas.
Por definição, o valor devido no mês $i$ será $f-i\cdot f/n$.
Então, se $j$ é o juros, temos a seguinte relação no mês $i$:


$$(f - i\cdot f/n)\cdot (1+j) - p_{i+1} = f - (i+1)\cdot f/n$$
$$\implies p_{i+1} = f+fj-i\cdot f/n-i\cdot f\cdot j/n -f + i\cdot f/n + f/n$$
$$\implies p_{i+1} = fj -i\cdot f \cdot j/n +  f/n$$
$$\implies p_{i+1} = f/n +fj(1-i/n)$$
$$\implies p_{i} = f/n +fj(1-(i-1)/n)$$

A fórmula acima fornece o valor a ser pago no mês $i$. Note que, se isolássemos
o fator multiplicativo da variável $i$, teríamos $-j(f/n)$. Esta é a propriedade
que mencionamos anteriormente: o valor das parcelas decai linearmente com taxa
igual ao juros ($j$) aplicado sobre a parcela "pura" ($f/n$).

## Tabela PRICE

A Tabela PRICE é definida por deixar constante as parcelas pagas todo mês.

Vamos olhar um exemplo novamente, porém mais simples: Suponha que
temos um financiamento de R$1,000.00 com juros de 1% ao mês que queremos
pagar em quatro parcelas.

Se não houvesse juros, pagaríamos R$250.00. Vamos ver quanto deveremos 
no final dos quatro meses, caso paguemos assim:

R$1,000.00 -> R$1,010.00 -> R$760.00\
R$760.00 -> R$767.60 -> R$517.60\
R$517.60 -> R$522.78 -> R$272.78\
R$272.78 -> R$275.51 -> R$25.51

Note que há um resto que continuamos devendo. É evidente que se pagássemos
um valor muito maior por mês, por exemplo R$600.00, quitaríamos a dívida
(e até teríamos crédito, caso simplesmente continuássemos pagando).

Pelo [Teorema do Valor Intermediário](https://pt.wikipedia.org/wiki/Teorema_do_valor_intermedi%C3%A1rio), existe um valor de pagamento mensal
que faz com que a dívida zere ao final dos quatro meses. Existe uma fórmula
para calcular esse valor, como veremos a seguir, mas não é relevante para o
conceito.
A título de curiosidade, nesse caso, o valor é R$256.28. (Faça a conta para
verificar!)

Vejamos agora como calcular a parcela mensal: seja $j$ o juros. Portanto,
o valor devido após incidir juros é obtido multiplicando por $(1+j)$, que
chamaremos de $r$. Seja o valor do financiamento $f$, e o valor que estamos
pagando por mês $p$. Seja $d_i$ o valor que devemos após o mês $i$.
Note que $d_0=f$. Finalmente, seja $n$ a quantidade de meses que queremos
que o financiamento dure.

Temos então que a cada mês, incidimos o juro sobre o valor que devemos,
e pagamos $p$. Isso é traduzido pela seguinte relação:

```math
d_{i+1} = d_i\cdot r-p
```

É fácil ver por indução que

```math
d_{i} = d_0\cdot r^i-p\cdot r^{i-1}-p\cdot r^{i-2}-...-p\cdot r^{}-p
```

Como queremos que $d_n=0$, temos a seguinte relação:

$$~~~~~~~~~~0 = d_0\cdot r^n-p\cdot \sum_{i=0}^{n-1}r^i$$
$$\implies p = \frac{d_0\cdot r^n}{(r^n -1)/(r-1)}$$
$$\implies p = d_0\cdot \frac{r ^n(r-1)} {r^n-1}$$
$$\implies p = d_0\cdot \frac{(r-1)} {1-1/r^n}$$
$$\implies p = d_0\cdot \frac{j} {1-(1+j)^{-n}}$$
