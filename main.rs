// PROYECTO: TAXI SOBERANO - BARCELONA AMB (CONSENSO DE RED)
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct NodoTaxi {
    licencia: String,
    tokens_privados: f64,
    aportacion_social: f64,
    esta_bloqueado: bool,
}

// NUEVA ESTRUCTURA: La Red que engloba a todos los taxis de la ciudad
struct RedTaxiSoberano {
    nodos: HashMap<String, NodoTaxi>,
}

impl RedTaxiSoberano {
    fn new() -> Self {
        RedTaxiSoberano { nodos: HashMap::new() }
    }

    // Registrar un nuevo compañero en la cadena de bloques
    fn registrar_nodo(&mut self, taxi: NodoTaxi) {
        self.nodos.insert(taxi.licencia.clone(), taxi);
    }

    // EL ESCUDO DE CONSENSO: Procesar un gasto requiriendo la firma de otro nodo vivo
    fn procesar_gasto_con_consenso(
        &mut self,
        licencia_solicitante: &str,
        licencia_guardian: &str,
        importe: f64,
        firma_solicitante: bool,
        firma_guardian: bool,
    ) -> Result<String, String> {
        
        // 1. Verificar si las dos firmas físicas se han presentado al revólver de seguridad
        if !firma_solicitante || !firma_guardian {
            // Si hay un intento de fraude, la red bloquea al nodo solicitante de inmediato
            if let Some(taxi) = self.nodos.get_mut(licencia_solicitante) {
                taxi.esta_bloqueado = true;
            }
            return Err(format!(
                "🚨 ALERTA DE SEGURIDAD: Intento de movimiento sin doble firma. Nodo {} BLOQUEADO por la red.",
                licencia_solicitante
            ));
        }

        // 2. Comprobar si el Guardián existe realmente y está activo en Barcelona
        if !self.nodos.contains_key(licencia_guardian) {
            return Err(format!("🔴 RECHAZADO: El nodo guardián [{}] no existe en la red.", licencia_guardian));
        }

        // 3. Validar los fondos y aplicar el movimiento matemático
        if let Some(taxi) = self.nodos.get_mut(licencia_solicitante) {
            if taxi.esta_bloqueado {
                return Err("🔴 RECHAZADO: Este nodo está bloqueado y sus fondos están congelados.".to_string());
            }
            if taxi.tokens_privados >= importe {
                taxi.tokens_privados -= importe;
                Ok(format!(
                    "🟢 CONSENSO LOGRADO: Nodo [{}] autoriza {} TEU. Validado por el Guardián [{}].",
                    licencia_solicitante, importe, licencia_guardian
                ))
            } else {
                Err("🔴 RECHAZADO: Fondos insuficientes en el monedero.".to_string())
            }
        } else {
            Err("🔴 RECHAZADO: El nodo solicitante no está registrado.".to_string())
        }
    }
}

fn main() {
    println!("--- CONFIGURANDO RED DE HONOR: TAXI BARCELONA ---\n");
    let mut red_amb = RedTaxiSoberano::new();

    // REGISTRO DE NODOS: Colocamos los tres pilares de acero en la ciudad
    red_amb.registrar_nodo(NodoTaxi {
        licencia: "TAXI-VICENTE".to_string(),
        tokens_privados: 1000.0,
        aportacion_social: 0.0,
        esta_bloqueado: false,
    });

    red_amb.registrar_nodo(NodoTaxi {
        licencia: "TAXI-JEFE".to_string(), // El Guardián de confianza
        tokens_privados: 2500.0,
        aportacion_social: 500.0,
        esta_bloqueado: false,
    });

    red_amb.registrar_nodo(NodoTaxi {
        licencia: "TAXI-ANDRES".to_string(), // Otro compañero de la parada
        tokens_privados: 800.0,
        aportacion_social: 50.0,
        esta_bloqueado: false,
    });

    // Imprimir el censo inicial para verificar que la red ve a los tres
    println!("Nodos activos en la red: {} taxis vigilando.", red_amb.nodos.len());
    println!("--------------------------------------------------");

    // PRUEBA 1: Vicente va al taller y el Jefe le firma digitalmente al lado
    println!("Escenario 1: Vicente solicita 300 TEU para el taller. El Jefe firma.");
    match red_amb.procesar_gasto_con_consenso("TAXI-VICENTE", "TAXI-JEFE", 300.0, true, true) {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{}", err),
    }
    println!("Tokens restantes de Vicente: {} TEU\n", red_amb.nodos.get("TAXI-VICENTE").unwrap().tokens_privados);

    // PRUEBA 2: Un "intruso" intenta retirar fondos simulando la firma de Vicente pero SIN la firma del Jefe
    println!("Escenario 2: Intento de hackeo. Retirar 100 TEU sin la firma del Jefe...");
    match red_amb.procesar_gasto_con_consenso("TAXI-VICENTE", "TAXI-JEFE", 100.0, true, false) {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{}", err),
    }

    // PRUEBA 3: Comprobar si el escudo de la red ha bloqueado el taxi de Vicente por seguridad
    println!("\nEscenario 3: Intentando operar tras el ataque...");
    match red_amb.procesar_gasto_con_consenso("TAXI-VICENTE", "TAXI-JEFE", 50.0, true, true) {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{}", err),
    }
}
