mod tipos;

use tipos::*;

use std::collections::HashMap;
use std::time;
use std::thread::sleep;

fn main() {
    // Cria os aeroportos
    let mut lista_aeroportos = HashMap::<String, Aeroporto>::new();

    let aero1 = Aeroporto::new("Guarulhos");
    lista_aeroportos.insert(aero1.nome.clone(), aero1.clone());
    let aero = Aeroporto::new("Congonhas");
    lista_aeroportos.insert(aero.nome.clone(), aero.clone());
    let aero = Aeroporto::new("Hercílio Luz");
    lista_aeroportos.insert(aero.nome.clone(), aero.clone());

    // Cria os Avioes
    let mut lista_aviao = Vec::<Aviao>::new();

    let maq = Aviao::new(aero.nome.clone());
    lista_aviao.push(maq);
    let maq = Aviao::new(aero.nome.clone());
    lista_aviao.push(maq);
    let maq = Aviao::new(aero.nome.clone());
    lista_aviao.push(maq);
    let maq = Aviao::new(aero.nome.clone());
    lista_aviao.push(maq);
    let maq = Aviao::new(aero.nome.clone());
    lista_aviao.push(maq);

    // Inicializa eles pela primeira vez
    lista_aviao[0].levantar_voo(String::from("Guarulhos")).unwrap();
    lista_aviao[1].levantar_voo(String::from("Guarulhos")).unwrap();
    lista_aviao[2].levantar_voo(String::from("Congonhas")).unwrap();
    lista_aviao[3].levantar_voo(String::from("Congonhas")).unwrap();
    lista_aviao[4].levantar_voo(String::from("Hercílio Luz")).unwrap();

    let turno = time::Duration::new(1, 0);

    // Entra no Loop de simulação
    loop {
        lista_aviao.reverse();

        dbg!(&lista_aviao);
        dbg!(&lista_aeroportos);

        sleep(turno);

        for a in lista_aviao.iter_mut() {
            let dest = check_destino(&a.get_local(), &lista_aeroportos).unwrap();

            match a.get_destino() {
                Ok(q) => {
                    match a.pousar(q, &mut lista_aeroportos) {
                        Ok(()) => {},
                        Err(()) => {

                        }
                    }
                }
                Err(_) => {
                    match a.levantar_voo(dest) {
                        Ok(s) => {
                            let mut a = lista_aeroportos.get_mut(&s).unwrap();
                            a.pistas += 1;
                            assert!(a.pistas < 3);
                        },
                        Err(()) => {
                        }
                    }
                }
            }
        }
    }
}
