use std::collections::HashMap;
use std::time;

#[derive(Debug, Clone)]
struct Aeroporto {
    nome: String,
    pistas: i8,
    tempo_para_liberar: time::Duration,
}

impl Aeroporto {
    fn new(n: &str) -> Aeroporto {
        Aeroporto {
            nome: String::from(n),
            pistas: 2,
            tempo_para_liberar: time::Duration::new(0, 0),
        }
    }
}

#[derive(Debug)]
enum AviaoEstado {
    Voando {
        tempo_chegada: time::Duration,
        combustivel_sobra: time::Duration,
        destino: String
    },
    Estacionado {
        tempo_saida: time::Duration,
        local: String
    },
    EsperandoAr {
        combustivel_sobra: time::Duration,
        destino: String
    },
}

#[derive(Debug)]
struct Aviao {
    estado: AviaoEstado
}

impl Aviao {
    fn new(a: String) -> Aviao {
        Aviao {
            estado: AviaoEstado::Estacionado{
                tempo_saida: time::Duration::new(0, 0),
                local: a
            },
        }
    }

    fn levantar_voo(&mut self, destino: String, l_aero: &HashMap<String, Aeroporto>) {
        self.estado = match self.estado {
            AviaoEstado::Estacionado {..} => AviaoEstado::Voando{
                tempo_chegada: time::Duration::new(0, 0),
                combustivel_sobra: time::Duration::new(0, 0),
                destino: destino
            },
            _ => panic!("Impossível levantar voo")
        }
    }

    fn pousar(&mut self, a: String, l_aero: &mut HashMap<String, Aeroporto>) {
        let mut a = l_aero.get_mut(&a).unwrap();

        if a.pistas < 1 {
            self.estado = match self.estado {
                AviaoEstado::Voando {..} => AviaoEstado::EsperandoAr{
                    combustivel_sobra: time::Duration::new(0, 0),
                    destino: a.nome.clone()
                },
                _ => panic!("Impossível pousar")
            };
        } else {
            self.estado = match self.estado {
                AviaoEstado::Voando {..} => AviaoEstado::Estacionado{
                    tempo_saida: time::Duration::new(0, 0),
                    local: a.nome.clone()
                },
                AviaoEstado::EsperandoAr {..} => AviaoEstado::Estacionado{
                    tempo_saida: time::Duration::new(0, 0),
                    local: a.nome.clone()
                },
                _ => panic!("Impossível pousar")
            };
            a.pistas -= 1;
        }
    }
}

fn main() {
    let mut lista_aeroportos = HashMap::<String, Aeroporto>::new();

    let aero = Aeroporto::new("Guarulhos");
    lista_aeroportos.insert(aero.nome.clone(), aero.clone());
    let aero = Aeroporto::new("Congonhas");
    lista_aeroportos.insert(aero.nome.clone(), aero.clone());

    let mut maq1 = Aviao::new(aero.nome.clone());
    let mut maq2 = Aviao::new(aero.nome.clone());
    let mut maq3 = Aviao::new(aero.nome.clone());

    maq1.levantar_voo(aero.nome.clone(), &lista_aeroportos);
    maq1.pousar(String::from("Congonhas"), &mut lista_aeroportos);

    maq2.levantar_voo(aero.nome.clone(), &lista_aeroportos);
    maq2.pousar(String::from("Congonhas"), &mut lista_aeroportos);

    maq3.levantar_voo(aero.nome.clone(), &lista_aeroportos);
    maq3.pousar(String::from("Congonhas"), &mut lista_aeroportos);

    dbg!(&maq1);
    dbg!(&maq2);
    dbg!(&maq3);

}
