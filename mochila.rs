use std::collections::HashSet;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Item {
    peso: usize,
    valor: usize
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(peso: {}, valor: {})", self.peso, self.valor)
    }
}

impl Item {
    fn new(peso_dado: usize, valor_dado: usize) -> Item{
        return Item {peso: peso_dado, valor: valor_dado};
    }
    
}

struct Mochila {
    capacidade: usize,
    ocupacao: usize,
    valor_total: usize,
    itens_pegos: HashSet<Item>
}

impl Mochila {
    fn duplica(&mut self) -> Mochila{
        Mochila{capacidade: self.capacidade, ocupacao: self.ocupacao, itens_pegos: self.itens_pegos.iter().cloned().collect(), valor_total: 0}
    }
    fn imprime(&mut self){
        println!("Itens na melhor mochila:");
        for item in &self.itens_pegos {
            println!("{}", item);
        }
    }
}



struct ResolveMochila{
    itens_disponiveis: Vec<Item>,
    mochila: Mochila,
    estados_mochila: HashSet<(usize, usize, usize)>,
    maior_valor: usize,
    melhor_solucao: Mochila
}

impl ResolveMochila{
    fn new(itens_disponiveis_dados: Vec<Item>, capacidade_mochila: usize) -> ResolveMochila{
        return ResolveMochila{itens_disponiveis: itens_disponiveis_dados, 
                               estados_mochila: HashSet::new(), 
                               mochila: Mochila{capacidade: capacidade_mochila, ocupacao: 0, itens_pegos: HashSet::new(), valor_total: 0},
                               maior_valor: 0,
                               melhor_solucao: Mochila{capacidade: capacidade_mochila, ocupacao: 0, itens_pegos: HashSet::new(), valor_total: 0}
                             };
    }

    fn resolve(&mut self){
        self.backtrack();
    }

    fn backtrack(&mut self){
        let estado_atual: (usize, usize, usize) = (self.mochila.ocupacao, self.mochila.itens_pegos.len(), self.mochila.valor_total);
        if self.estados_mochila.contains(&estado_atual){
            return;
        } else {
            if self.mochila.valor_total > self.maior_valor{
                self.melhor_solucao = self.mochila.duplica();
                self.maior_valor = self.mochila.valor_total;
            }
            self.estados_mochila.insert(estado_atual);
        }
        
        for indice in 0..(self.itens_disponiveis.len()) {
            let item = self.itens_disponiveis[indice];
            if (item.peso + self.mochila.ocupacao) < self.mochila.capacidade {
                self.itens_disponiveis.remove(indice);
                self.mochila.itens_pegos.insert(item);
                self.mochila.ocupacao = self.mochila.ocupacao + item.peso;
                self.mochila.valor_total = self.mochila.valor_total + item.valor;
                self.backtrack();
                self.itens_disponiveis.insert(indice, item);
                self.mochila.itens_pegos.remove(&item);
                self.mochila.ocupacao = self.mochila.ocupacao - item.peso;
                self.mochila.valor_total = self.mochila.valor_total - item.valor;

            }
        }
    }
}

fn main() {
    let mut itens_disponiveis = vec![Item::new(5, 2), Item::new(20, 12), Item::new(14, 14), Item::new(23, 18), Item::new(32, 30)];
    let mut calculador = ResolveMochila::new(itens_disponiveis, 50);
    calculador.resolve();
    calculador.melhor_solucao.imprime();
}
