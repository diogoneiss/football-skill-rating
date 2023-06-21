# Projeto Ciência de Dados Aplicada ao Futebol e Computação Natural

TODOs

- Achar uma lógica melhor que utils, isso ta confuso, talvez models? Não sei o jeito mais rustáceo
- Mover Season e League pra fora de elo, elas não são relacionadas a elo
- Extrair types para um arquivo types, sla, como no caso da EloTable 



- [Projeto Ciência de Dados Aplicada ao Futebol e Computação Natural](#projeto-ciência-de-dados-aplicada-ao-futebol-e-computação-natural)
  - [Problemas atuais](#problemas-atuais)
  - [Como o elo funciona normalmente](#como-o-elo-funciona-normalmente)
  - [Como o elo funciona no futebol](#como-o-elo-funciona-no-futebol)
  - [Base de dados](#base-de-dados)
  - [Modelos de avaliação de habilidade](#modelos-de-avaliação-de-habilidade)
  - [Parâmetros de experimentação do Elo](#parâmetros-de-experimentação-do-elo)
  - [Hiperparâmetros](#hiperparâmetros)
  - [Modelagem do problema como algoritmo evolucionário](#modelagem-do-problema-como-algoritmo-evolucionário)
    - [Mutação](#mutação)
  - [Função de fitness/avaliação de qualidade](#função-de-fitnessavaliação-de-qualidade)
  - [Como lidar com rebaixamentos e promoções](#como-lidar-com-rebaixamentos-e-promoções)
  - [Experimentação](#experimentação)
  - [Implementação](#implementação)


## Problemas atuais

- Como incorporar intervalos de confiança no nosso algoritmo, evitando que um evento improvável se propague no modelo
- Devemos misturar com algum algoritmo de predição de resultados na fórmula de atualização de pesos?
- Como incorporar um modelo de predição de gols (xG) no modelo sem enviesá-lo excessivamente
- Como criar benchmarks com algoritmos reais
- Como ajustar a função de elo para previsão de empates

## Como o elo funciona normalmente

Modelos de *************skill rating************* pretendem atribuir uma nota para cada time, seguindo alguma distribuição estatística, de modo que possa ser estimada com alguma precisão o resultado de uma partida entre diferentes jogadores, e que essa propriedade se mantenha ao longo do tempo.

Ele foi criado para o Xadrez e se disseminou para diversos jogos, sendo utilizado principalmente para balanceamento de jogos online.

O ponto principal do elo é que a diferença de elos entre dois indivíduos (**dr**) prevê o resultado de uma partida entre eles. No caso do elo clássico, uma diferença de 100 pontos implica em 64% de vitória, uma de 200 74%

| Probabilidade | dr |
| --- | --- |
| 1 | +800 |
| 0.99 | +677 |
| 0.9 | +366 |
| 0.8 | +240 |
| 0.7 | +149 |
| 0.6 | +72 |
| 0.5 | 0 |
| 0.4 | −72 |
| 0.3 | −149 |
| 0.2 | −240 |
| 0.1 | −366 |
| 0.01 | −677 |
| 0 | −800 |

A formula para o resultado esperado de *A*, baseando-se nos ratings de A e B ($R_a \text{ e } R_b$) é

$$
E_a = \frac{1}{1+10^{(R_b - R_a)/400}}
$$

Temos também que o novo ranking de A, baseado no anterior, após a partida é dado por

$$
R_{t} = R_{t-1} + \Delta Elo_{t-1}
$$

$$
\Delta Elo_t = K \cdot (S_t - E_t)
$$

Com $S_a$ sendo o *****score***** real da partida, geralmente 1 para vitória, 0.5 para empate e 0 para derrota, $E_a$ o esperado e $K$ uma constante de ajuste.

## Como o elo funciona no futebol

O elo dos times é recalculado ao final de cada partida, dependendo de vitória, derrota ou empate, levando em consideração os seguintes fatores

- O elo absoluto anterior dos times
- A diferença (**dr**) de elo entre os times
- O resultado
- O resultado esperado, previsto por algum modelo, seja o próprio estimador do elo, distribuição Poisson bivariada ou cadeia de markov
- A presença ou não da métrica de ofensividade, denominada tilt, que é baseada na diferença de gols feitos e esperados da partida e os valores anteriores

$$
\text{TiltA}_{\text{t}} = 0.98 \cdot \text{TiltA}_{\text{t-1}}
+ 0.02 \cdot \frac{\text{TotalGoals}}{\text{TiltB}_{\text{t-1}} \cdot \text{ExpectedGoals}}
$$

- Fator de ajuste *K* utilizando a diferença de gols, baseado no artigo
    
    Hvattum, L.M., Arntzen, H.: Using ELO ratings for match result prediction in association football. Int J Forecasting 26(3), 460–470 (Jul 2010).
    [https://doi.org/10.1016/j.ijforecast.2009.10.002](https://doi.org/10.1016/j.ijforecast.2009.10.002)
    

$$
k = k_0 \cdot w_i \cdot (1 + \Delta G)^{\gamma}\\
\begin{align*}
\\
&\text{where}\\
&k\text{ is the elo adjustment factor,}\\
&k_0\text{ is the \textit{recentness} factor}\\ 

&w_i \gt 0\text{ is the weight of the $i$th competition,}\\
&\Delta G\text{ the goal difference,}\\
&\gamma\text{ goal margin impact}
\end{align*}
$$

- $hfa$, home field advantage ou vantagem do time em casa, constante ou variável, que é adicionada ou multiplicada como peso para o time em casa. No caso de ser variável, a ajustamos de acordo com a proporção de vitória para times em casa e fora de casa.
    
    $$
    
    \mathrm{hfa}_{t+1} = \mathrm{hfa}_t + 0.075 \sum_{i=1}^{n} (\Delta \mathrm{Elo}_{h,i} - \Delta \mathrm{Elo}_{a,i})
    \\
    \text{
    where \(n\) is the number of matches in the season.
    }
    $$
    

Esses parâmetros são incorporados nas duas fórmulas vistas anteriormente

$$
E_a = \frac{1}{1+10^{(R_b - R_a)/400}}
$$

$$
\Delta Elo_{\text{margin}} = \Delta Elo_{\text{1 goal}} \cdot \sqrt{\text{margin}}
\\
\Delta Elo_{\text{1 goal}} = \frac{\Delta Elo_{base}}{\sum \sqrt{\text{margin}} \cdot \frac{p_{\text{margin}}}{p_{\text{1X2}}}}
$$

## Base de dados

Utilizamos dados de súmula para análise, além do valor do time no momento da partida, que ainda será incorporado na base de dados. Tendo em mãos os dados de súmula de uma temporada, conseguimos construir a tabela de classificação e derivar os pontos, baseando-se nos gols feitos e resultado da partida

| HomeTeam | AwayTeam | Full time home goals | Full time away goals | Full time result | Season |
| --- | --- | --- | --- | --- | --- |
| Guarani | Vasco | 4 | 2 | H | 2003 |
| Athletico-PR | Gremio | 2 | 0 | H | 2003 |
| Flamengo | Coritiba | 1 | 1 | D | 2003 |
| Goias | Paysandu | 2 | 2 | D | 2003 |
| Internacional | Ponte Preta | 1 | 1 | D | 2003 |
| Criciuma | Fluminense | 2 | 0 | H | 2003 |

Ainda será necessário incorporar data da partida, já que queremos colocar a frequência de jogo como parâmetro do modelo.

Desconsideraremos partidas e amistosos fora da liga para efeitos de comparação, já que aumentariam muito a complexidade do modelo, já que são influenciados por outros fatores, como altitude, distância e problemas de treinamento do time. 

Após acharmos uma métrica boa de elo, podemos fazer estudos da evolução do elo no histórico total de partidas do time

## Modelos de avaliação de habilidade

Experimentaremos diversos modelos e configurações para avaliação de previsão de resultados no Futebol, utilizando algoritmos de avaliação de habilidade em competições ********pairwise********. Pensamos nos seguintes modelos:

Baseados no modelo clássico de Elo, sofrendo poucas modificações

- Elo
- FIFA Men's World Ranking
- USCF (US Chess Federation)
- EGF (European Go Federation)
- DWZ (Deutsche Wertungszahl)

Baseados no ******Glicko****** (introduz variação de *******rating******* no modelo de Elo)

- Glicko
- Glicko-2
- Sticko
- Glicko-Boost

Outros

- Ingo

## Parâmetros de experimentação do Elo

Nossa ideia baseia-se em definir os parâmetros da função de elo. Para isso, pretendemos variar os parâmetros da função e verificar sua qualidade

- Parâmetro capaz de incorporar a vantagem do time de casa na predição de resultados
- Variação do valor $K$ (peso da partida no torneio) para qualificação de resultados de acordo com a liga e divisão
- Variar $K$ por partida conforme torneio avança de acordo com alguma função
- Como escolher o valor de $K$ ao longo de etapas eliminatórias
- Variação do fator de escala da distribuição logística (usualmente $400$). Pouco espaço para melhora, porém pode ser interessante em outros modelos de mensuração de habilidade, como Glicko.
- Variar a base da distribuição logística, frequentemente $10$
- Usar ou não diferença de gols na função com algum peso
- Usar ou não ******tilt******, medida de ofensividade baseada na diferença de gols  $g_{scored} - g_{expected}$
- Performance de modelos alternativos de ************skill rating************, como Glicko
- Diferença de valor entre os times: como introduzir orçamento como parâmetro
- Introduzir pesos no cálculo de probabilidade baseados na partida ser um *********clássico********* ou não, baseando-se na hipótese que clássicos são mais disputados, logo os times disputariam mais a partida, aumentando probabilidade de empate
- Como incluir a frequência de jogos de um time no modelo: times que jogam mais vezes na semana performam pior
- Como contabilizar partidas amistosas/campeonatos com times fora da liga

O modelo usará grande parte dos parâmetros de experimentação, com alguns parâmetros inicialmente desligados. Após a coleta de informação para um número legal de gerações e combinações de hiperparâmetros, habilitaremos o conjunto total de pontos de experimentação.

## Hiperparâmetros

- Quantas runs iniciais serão necessárias para “contextualização” do modelo, denominado por $t$
- Incluir ou não orçamento dos times no momento de cada partida
- *****Step***** mínimo de variação dentro da mutação
- *Step* máximo
- Taxa de mutação, crossover, elite e reprodução
- Como garantir diversidade no modelo: estudar implementação de niching
- Estimar critérios de parada

## Modelagem do problema como algoritmo evolucionário

Para modelar uma solução baseada em computação natural criaremos diferentes conjuntos de coeficientes da função de elo e hiper parâmetros para formar nosso ********genótipo,******** ou seja, o indivíduo será uma tupla de valores que guiarão a execução do modelo.

Se quisermos estudar variação de parâmetros *******k, w******* e **dr**, por exemplo, criaremos um genótipo variando os valores $(k, w, dr)$, e o indivíduo 1 poderia ser representado como $(0, 100, 0.1)$ e o indivíduo 2 $(0, 100, 0.1)$, fazendo isso para todos os indivíduos na população.

Em seguida, é feita alguma estratégia de seleção, que removerá da população soluções ruins, baseando-se em uma função de *******fitness,******* que avalia a qualidade da solução. 

Por fim, geramos novos indivíduos com mutações, cruzamento e clonagem dos indivíduos sobreviventes do passo anterior, e isso se repetirá até um critério de convergência ou máximo de populações atingida. Será importante limitar os operadores genéticos ao domínio permitido, de forma que variem os números dentro de intervalos viáveis para que a função de elo seja viável.

Não planejamos utilizar competição multi espécie, pensamos em executar o algoritmo em configurações separadas para cada modelo de elo, já que eles diferem muito em si.

### Mutação

A mutação deve ser feita escolhendo um conjunto de variáveis para ser mutada, e para cada uma delas, determinaremos o intervalo permitido.

Com o intervalo permitido em mãos, poderemos escolher uma de duas estratégias:

- Aumentar ou diminuir de acordo com um

## Função de fitness/avaliação de qualidade

Como gostaríamos de verificar o elo como estimador de vitória, precisamos usá-lo na predição de resultados de jogos, evoluindo a série temporal com o resultado da partida determinada pela probabilidade de vitória do elo, para ao final da série temporal verificarmos o quão bem ela performou.

Para isso, teremos o conjunto de elos no estado inicial $t_0$, onde simularemos partidas. Para dois times $A$ e $B$, A diferença de elo prevê $p_{win}$ para $A$ de $55\%$. De acordo com essa probabilidade, determinaremos um vencedor para a partida e ajustaremos o elo de ambos, de acordo com o resultado da partida, e repetimos isso para todos os times na liga. Iremos até o tempo $t_n$, aplicando esse processo indutivamente, de $t_0 \text{ até }t_{n-1}$. Para evitar que a geração de números aleatórios para o resultado da partida influencie o resultado, computaremos diversas vezes, gerando um intervalo de confiança de valor arbitrário.

Por fim, para fazer a avaliação em si, precisamos pensar em como verificar se a probabilidade foi boa: verificando se os resultados finais foram precisos ou se as previsões em si foram boas.

Como sugerido pelo artigo [inserir ref do que ele passou], faremos Ranked Probability Score, comparando as probabilidades que o elo gerou e as probabilidades que um modelo de elo treinado com dados reais estimaria. 

TODO:

Um problema que temos é: se o modelo de elo que temos é uma porcaria, as estimativas do modelo treinado com dados reais também serão, então precisamos ver também a tabela final e comparar a proximidade com o real. Talvez isso não seja necessário, já que o algoritmo de elo deveria prever bem e, se ele prevê mal o resultado, os elos serão atualizados de maneira incorreta e as probabilidades ficarão desreguladas.

$$
P_{win} = \frac{1}{10^{-dr/400} + 1}
$$

Nesse ponto, computaremos os pontos de todos os times e a tabela de classificação, comparando os resultados do modelo evolucionário com os resultados reais para essa temporada. A distância entre eles será nossa função de fitness, com a tarefa do algoritmo se resumindo a minimizar a distância do modelo evolucionário com algum modelo de comparação, seja ele o real ou o previsto com algum modelo estado da arte de aprendizado de máquina.

Tentaremos criar um modelo evolucionário melhor que o estado da arte, aproximando-se o máximo possível do real. Esperamos que nossas soluções aceitas respeitem a inequação

$$
score_{benchmark} \leq score_{elo} \lt real
$$

de forma que sejam capazes de ganhar de implementações inocentes de previsão de resultado, como regressões *Poisson* simples

## Como lidar com rebaixamentos e promoções

Pensaremos estratégias para tornar o modelo resistente a rebaixamento entre divisões, avaliando diferentes heurísticas, como

- Troca de valor absoluto do elo entre times trocando de divisões
- Inflação/deflação do elo ao trocar de divisões
- Aumentar/diminuir a volatilidade de elo, introduzindo fatores constantes ou variáveis de ajuste por um número de partidas inicial
- Variar *K* entre partidas de diferentes divisões, porém sem modificar nada

## Experimentação

Para medir a qualidade de cada genótipo, isto é, nosso modelo, precisaremos fazer *backtesting*: 

Como o elo modela probabilidade de vitória, usaremos essa informação para evoluir uma solução num número específico de anos, para em seguida comparar com o resultado real e medir a distância dele: quanto menor, melhor. Isso será nosso *******fitness*******

Então, atribuiremos valores base de elo a cada time no modelo e executaremos um número *t* de iterações reais, de modo que o elo aproxime um valor inicial legal para experimentação, com $0 \leq t \leq n - 1$ , sendo um parâmetro de experimentação do modelo. Isso garantirá contextualização para o modelo, de modo que ele tenha algum parâmetro de base para próximas temporadas.

Precisamos descobrir também qual a melhor maneira de calcular o fitness entre as $n$ temporadas

- Singular: Calcular a fitness da solução na temporada atual e comparar sua proximidade com o real

$$

\begin{align*}
&\exists n \in \{1, 2, ..., t\}, \\
& elo_{n} = evolve(elo_{n-1})\\
&fitness = f(elo_n) \\
&\text{where } evolve \text{ is a full season simulated elo update} \\
\end{align*}

$$

- Acumulada: Calcular a fitness em várias temporadas seguidas a partir de uma temporada inicial $t_0$, evoluindo até $t_n$, momento que compararemos com o real. Necessário usar rebaixamento de times entre temporadas.
    
    $$
    
    \begin{align*}
    &\forall n \in \{1, 2, ..., t\}, \\
    & elo_{n} = evolve(elo_{n-1})\\
    &fitness = f(elo_t) \\
    &\text{where } evolve \text{ is a full season simulated elo update} \\
    \end{align*}
    
    $$
    
- Temporal: Calcularemos a fitness em várias temporadas seguidas a partir de uma temporada inicial $t_0$, evoluindo até $t_n$, comparando com o real ao final de cada $t$, agregando as $n$ fitness por temporada em uma final do modelo. Necessário tomar cuidado com as times presentes em cada divisão por temporada, já que o modelo real provavelmente terá times rebaixados diferentes do modelo real.
    
    A função de agregação inicialmente será **************************exponential moving average**************************, de forma que erros no começo sejam mais significativos, dando menos peso conforme o tempo passa, compensando a propagação de erros
    
    $$
    
    \begin{align*}
    &\forall n \in \{1, 2, ..., t\}, \\
    & elo_{n} = evolve(elo_{n-1})\\
    &\text{calculate } F_n = \{f_1, f_2, ..., f_m\}, \\
    &\text{where } evolve \text{ is a full season simulated elo update} \\
    &\text{where } f_i \text{ is the fitness of the } i\text{-th element} \\
    &\\
    &\textit{fitness} = F_{\text{agg}} = \bigcup_{n} F_n
    \end{align*}
    
    $$
    

Optaremos pela forma ********Temporal********, já que a 1ª forma é enviesada para aquele conjunto de dados, podendo gerar overfitting, e a 2ª propaga o erro demais.

## Implementação

Faremos o código em ****Rust****, devido aos requisitos de performance, já que rodaremos uma quantidade elevadíssima de combinações de parâmetros, algo inviável em Python. Poderíamos utilizar a biblioteca *****Numba,***** mas a dificuldade de debugging e problemas de tipagem pioram muito o modelo.
