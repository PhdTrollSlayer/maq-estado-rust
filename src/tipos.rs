use std::collections::HashMap;

use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Aeroporto {
    pub nome: String,
    pub pistas: i8,
}

impl Aeroporto {
    pub fn new(n: &str) -> Aeroporto {
        Aeroporto {
            nome: String::from(n),
            pistas: 2,
        }
    }
}

#[derive(Debug)]
pub enum AviaoEstado {
    Voando {
        destino: String
    },
    Estacionado {
        local: String
    },
    EsperandoAr {
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
                local: a
            },
        }
    }

    pub fn levantar_voo(&mut self, destino: String) -> Result<String, ()> {
        let mut s;
        self.estado = match &self.estado {
            AviaoEstado::Estacionado {local} => {

                s = local.clone();
                AviaoEstado::Voando{
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
                AviaoEstado::Voando {destino: _} => AviaoEstado::EsperandoAr{
                    destino: a.nome.clone()
                },
                _ => return Err(())
            };
        } else {
            self.estado = match self.estado {
                AviaoEstado::Voando {..} => AviaoEstado::Estacionado{
                    local: a.nome.clone()
                },
                AviaoEstado::EsperandoAr {..} => AviaoEstado::Estacionado{
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
            AviaoEstado::Voando{destino} => {
                return Ok(destino.clone())
            },
            AviaoEstado::EsperandoAr{destino} => {
                return Ok(destino.clone())
            }
            _ => {return Err(())}
        }
    }

    pub fn get_local(&self) -> String {
        match &self.estado {
            AviaoEstado::Estacionado {local} => {
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
