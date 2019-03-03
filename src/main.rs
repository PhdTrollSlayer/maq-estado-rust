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

    fn levantar_voo(&mut self, destino: String, l_aero: &mut HashMap<String, Aeroporto>) -> Result<(), ()> {
        self.estado = match &self.estado {
            AviaoEstado::Estacionado {tempo_saida, local} => {
                let mut a = l_aero.get_mut(local).unwrap();
                a.pistas += 1;
                AviaoEstado::Voando{
                    tempo_chegada: time::Duration::new(0, 0),
                    combustivel_sobra: time::Duration::new(0, 0),
                    destino: destino
                }
            },
            _ => return Err(())
        };

        return Ok(())
    }

    fn pousar(&mut self, a: String, l_aero: &mut HashMap<String, Aeroporto>) -> Result<(), ()> {
        let mut a = l_aero.get_mut(&a).unwrap();

        if a.pistas < 1 {
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
            AviaoEstado::Voando {destino, tempo_chegada: _, combustivel_sobra: _} => {return Ok(destino.clone())},
            _ => {return Err(())}
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

    lista_aviao[0].levantar_voo(aero.nome.clone(), &mut lista_aeroportos).unwrap();
    lista_aviao[1].levantar_voo(aero.nome.clone(), &mut lista_aeroportos).unwrap();
    lista_aviao[2].levantar_voo(aero.nome.clone(), &mut lista_aeroportos).unwrap();

    let turno = time::Duration::new(1, 0);
    loop {
        let mut rng = thread_rng();

        sleep(turno);

        for a in lista_aviao.iter_mut() {
            // gera ação
            let u: i8 = rng.gen_range(0, 2);

            // gera destino
            let i: i8 = rng.gen_range(0, lista_aeroportos.len() as i8);
            let mut dest = String::new();

            for (p, l) in lista_aeroportos.iter().enumerate() {
                if p == i as usize {
                    dest = l.0.to_string();

                }
            }

            match u {
                // levantar voo
                0 => {
                    match a.levantar_voo(dest, &mut lista_aeroportos) {
                        Ok(()) => {},
                        Err(()) => {}
                    }
                },
                // pousar
                1 => {
                    match a.get_destino() {
                        Ok(q) => {
                            match a.pousar(q, &mut lista_aeroportos) {
                                Ok(()) => {},
                                Err(()) => {}
                            }
                        }
                        Err(_) => {}
                    };
                },
                _ => {}
            }

        }

        dbg!(&lista_aviao);

    }

    /*
    */

}
