use std::collections::HashMap;
use std::time;
use std::thread::sleep;

use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Aeroporto {
    pub nome: String,
    pub pistas: i8,
    pub tempo_para_liberar: time::Duration,
}

impl Aeroporto {
    pub fn new(n: &str) -> Aeroporto {
        Aeroporto {
            nome: String::from(n),
            pistas: 2,
            tempo_para_liberar: time::Duration::new(0, 0),
        }
    }
}

#[derive(Debug)]
pub enum AviaoEstado {
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
pub struct Aviao {
    estado: AviaoEstado
}

impl Aviao {
    pub fn new(a: String) -> Aviao {
        Aviao {
            estado: AviaoEstado::Estacionado{
                tempo_saida: time::Duration::new(0, 0),
                local: a
            },
        }
    }

    pub fn levantar_voo(&mut self, destino: String) -> Result<String, ()> {
        let mut s;
        self.estado = match &self.estado {
            AviaoEstado::Estacionado {tempo_saida: _, local} => {

                s = local.clone();
                AviaoEstado::Voando{
                    tempo_chegada: time::Duration::new(8, 0),
                    combustivel_sobra: time::Duration::new(10, 0),
                    destino: destino
                }
            },
            _ => return Err(())
        };

        return Ok(s);

    }

    pub fn pousar(&mut self, a: String, l_aero: &mut HashMap<String, Aeroporto>) -> Result<(), ()> {
        let mut a = l_aero.get_mut(&a).unwrap();

        if a.pistas == 0 {
            self.estado = match self.estado {
                AviaoEstado::Voando {combustivel_sobra, destino: _, tempo_chegada: _} => AviaoEstado::EsperandoAr{
                    combustivel_sobra: combustivel_sobra,
                    destino: a.nome.clone()
                },
                _ => return Err(())
            };
        } else {
            self.estado = match self.estado {
                AviaoEstado::Voando {..} => AviaoEstado::Estacionado{
                    tempo_saida: time::Duration::new(2, 0),
                    local: a.nome.clone()
                },
                AviaoEstado::EsperandoAr {..} => AviaoEstado::Estacionado{
                    tempo_saida: time::Duration::new(2, 0),
                    local: a.nome.clone()
                },
                _ => return Err(())
            };
            a.pistas -= 1;
        }
        Ok(())
    }

    pub fn get_destino(&self) -> Result<String, ()> {
        match &self.estado {
            AviaoEstado::Voando {destino, tempo_chegada: _, combustivel_sobra: _} => {
                return Ok(destino.clone())
            },
            _ => {return Err(())}
        }
    }

    pub fn get_local(&self) -> String {
        match &self.estado {
            AviaoEstado::Estacionado {local, tempo_saida: _} => {
                return local.clone()
            },
            _ => {String::new()}
        }
    }

}

pub fn check_destino(local: &String, lst: &HashMap<String, Aeroporto>) -> Result<String, ()> {
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
