use std::collections::HashMap;
use std::time;
use std::thread::sleep;

use rand::{thread_rng, Rng};

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

    fn levantar_voo(&mut self, destino: String) -> Result<String, ()> {
        let mut s = String::new();
        self.estado = match &self.estado {
            AviaoEstado::Estacionado {tempo_saida: _, local} => {

                s = local.clone();
                AviaoEstado::Voando{
                    tempo_chegada: time::Duration::new(0, 0),
                    combustivel_sobra: time::Duration::new(0, 0),
                    destino: destino
                }
            },
            _ => return Err(())
        };

        return Ok(s);

    }

    fn pousar(&mut self, a: String, l_aero: &mut HashMap<String, Aeroporto>) -> Result<(), ()> {
        let mut a = l_aero.get_mut(&a).unwrap();

        if a.pistas == 0 {
            self.estado = match self.estado {
                AviaoEstado::Voando {..} => AviaoEstado::EsperandoAr{
                    combustivel_sobra: time::Duration::new(0, 0),
                    destino: a.nome.clone()
                },
                _ => return Err(())
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
                _ => return Err(())
            };
            a.pistas -= 1;
        }
        Ok(())
    }

    fn get_destino(&self) -> Result<String, ()> {
        match &self.estado {
            AviaoEstado::Voando {destino, tempo_chegada: _, combustivel_sobra: _} => {
                return Ok(destino.clone())
            },
            _ => {return Err(())}
        }
    }

    fn get_local(&self) -> String {
        match &self.estado {
            AviaoEstado::Estacionado {local, tempo_saida: _} => {
                return local.clone()
            },
            _ => {String::new()}
        }
    }

}

fn check_destino(local: &String, lst: &HashMap<String, Aeroporto>) -> Result<String, ()> {
    let mut rng = thread_rng();

    loop {
        let i: i8 = rng.gen_range(0, lst.len() as i8);
        let mut dest = String::new();

        for (p, l) in lst.iter().enumerate() {
            if p == i as usize {
                dest = l.0.to_string();

            }
        }

        if !dest.eq(local) {
            return Ok(dest)
        }

    }
}

fn main() {
    let mut lista_aeroportos = HashMap::<String, Aeroporto>::new();

    let aero1 = Aeroporto::new("Guarulhos");
    lista_aeroportos.insert(aero1.nome.clone(), aero1.clone());
    let aero = Aeroporto::new("Congonhas");
    lista_aeroportos.insert(aero.nome.clone(), aero.clone());

    let maq1 = Aviao::new(aero.nome.clone());
    let maq2 = Aviao::new(aero.nome.clone());
    let maq3 = Aviao::new(aero.nome.clone());

    let mut lista_aviao = Vec::<Aviao>::new();
    lista_aviao.push(maq1);
    lista_aviao.push(maq2);
    lista_aviao.push(maq3);

    lista_aviao[0].levantar_voo(String::from("Guarulhos"));
    lista_aviao[1].levantar_voo(String::from("Guarulhos"));
    lista_aviao[2].levantar_voo(String::from("Congonhas"));

    let turno = time::Duration::new(2, 0);

    loop {

        sleep(turno);

        for a in lista_aviao.iter_mut() {
            let dest = check_destino(&a.get_local(), &lista_aeroportos).unwrap();

            match a.levantar_voo(dest) {
                Ok(s) => {
                    let mut a = lista_aeroportos.get_mut(&s).unwrap();
                    a.pistas += 1;
                    assert!(a.pistas < 3);
                },
                Err(()) => {
                    match a.get_destino() {
                        Ok(q) => {
                            match a.pousar(q, &mut lista_aeroportos) {
                                Ok(()) => {},
                                Err(()) => {

                                }
                            }
                        }
                        Err(_) => {
                        }
                    }

                }
            }

        }

        //dbg!(&lista_aviao);
        dbg!(&lista_aeroportos);

    }

    /*
    */

}
