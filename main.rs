// PROYECTO: TAXI /SOBERANO - BARCELONA AMB
// "Ayúdanos a regar la flor de la esperanza, desde nuestra humilde aportación"

use std::collections::HashMap;
use std::io::{self, Write}; // La herramienta para escuchar tu teclado

#[derive(Debug, Clone, PartialEq)]
enum TipoAviso {
    SosEmergencia,    // Alerta máxima (Auxilio en carretera)
    AvisoParada,      // Información útil de la jornada (Festivales, tráfico...)
}

#[derive(Debug, Clone)]
struct MensajeRed {
    id_emisor: String,       // Carnet IMET del taxista que avisa
    licencia_coche: String,  // Licencia vinculada para localizarlo
    tipo: TipoAviso,
    contenido: String,       // El texto del aviso ("¡Auxilio!", "Retención en Sónar"...)
    hora_registro: String,   // Para saber cuándo se lanzó
}

impl NodoTaxi {
    // ========================================================
    // PRIMERA PIEZA: LA FUNCIÓN DE AUTORIZAR GASTO (LA QUE YA TENÍAS)
    // ========================================================
    fn autorizar_gasto(
        &mut self, 
        importe: f64, 
        firma_biometrica_propia: bool, 
        firma_guardian: bool
    ) -> Result<String, String> {
        
        // El escudo contra "bandidos" remotos
        if firma_biometrica_propia && firma_guardian {
            if self.tokens_privados >= importe { // <--- Recuerda cambiar aquí "ahorro_tokens" por "tokens_privados"
                self.invertir_en_gasto(importe);
                Ok(format!("Pago de {} tokens autorizado con éxito. La unión hace la fuerza.", importe))
            } else {
                Err("Fondos insuficientes en el Token de la Esperanza.".to_string())
            }
        } else {
            Err("ALERTA: Intento de acceso sin doble validación física. Bloqueando fondos.".to_string())
        }
    }

    // Función auxiliar para restar los tokens de forma segura
    fn invertir_en_gasto(&mut self, importe: f64) {
        self.tokens_privados -= importe; // <--- Recuerda cambiar aquí también "ahorro_tokens" por "tokens_privados"
    }


    // ========================================================
    // SEGUNDA PIEZA: AQUÍ ENCAJAMOS LA NUEVA FUNCIÓN DEL TOKEN TEU
    // (Justo antes de que se cierre la llave grande del "impl")
    // ========================================================
    fn sembrar_token_util(&mut self, cantidad: f64, mensaje_esperanza: String) -> Result<String, String> {
        if self.esta_bloqueado {
            return Err("Operación denegada: Este nodo está bloqueado por seguridad.".to_string());
        }

        if self.tokens_privados >= cantidad {
            self.tokens_privados -= cantidad;
            self.aportacion_social += cantidad;
            Ok(format!(
                "🟢 SEMBRADO CON ÉXITO: Has aportado {} TEU a la red social. Mensaje grabado: '{}'", 
                cantidad, mensaje_esperanza
            ))
        } else {
            Err("🔴 ERROR: No tienes suficientes tokens TEU en tu bolsillo privado.".to_string())
        }
    }

} // <--- ¡CUIDADO! Esta es la llave de cierre final de todo el "impl". No la borres.  

fn main() {
    println!("--- Iniciando Nodo de Honor: Taxi Barcelona ---\n");
    
    // 1. Creamos el Taxi de Vicente con 1000 tokens TEU privados y 0 aportados inicialmente
    let mut taxi_vicente = NodoTaxi {
        licencia: "AMB-1234".to_string(),
        tokens_privados: 1000.0,
        aportacion_social: 0.0,
        guardian_asignado: "Jefe-Nodo-01".to_string(),
        esta_bloqueado: false,
    };

    println!("Estado inicial del taxi: {:?}", taxi_vicente);
    println!("--------------------------------------------------");

    // SIMULACIÓN 1: Vicente va al taller (Usa el escudo de doble firma)
    println!("Simulando operación en taller (Vicente + Guardián)...");
    match taxi_vicente.autorizar_gasto(250.0, true, true) {
        Ok(mensaje) => println!("🟢 ÉXITO: {}", mensaje),
        Err(error) => println!("🔴 ERROR: {}", error),
    }
    println!("Bolsillo privado de Vicente: {} TEU\n", taxi_vicente.tokens_privados);

    // SIMULACIÓN 2: ¡Momento Solidario! Vicente decide sembrar tokens para ayuda social
    println!("Simulando aportación voluntaria a la hucha comunitaria...");
    let mensaje_vicente = "Para el comedor social del barrio, que a nadie le falte un plato.".to_string();
    
    match taxi_vicente.sembrar_token_util(150.0, mensaje_vicente) {
        Ok(mensaje) => println!("{}", mensaje),
        Err(error) => println!("{}", error),
    }

    println!("\n--------------------------------------------------");
    println!("=== ESTADO FINAL DEL NODO ===");
    println!("Licencia: {}", taxi_vicente.licencia);
    println!("Bolsillo Privado: {} TEU", taxi_vicente.tokens_privados);
    println!("Sembrado en Ayuda Social: {} TEU", taxi_vicente.aportacion_social);
    println!("--------------------------------------------------");
}

    red_amb.registrar_taxista(NodoTaxi {
        carnet_imet: "IMET-9999".to_string(), // Tu compañero del segundo turno
        licencia_vinculada: "AMB-3931".to_string(), // ¡El mismo coche!
        tokens_privados: 500.0,
        aportacion_social: 20.0,
        esta_bloqueado: false,
    });

    // El carnet de tu Jefe (El Guardián Principal de la parada)
    red_amb.registrar_taxista(NodoTaxi {
        carnet_imet: "IMET-0001".to_string(),
        licencia_vinculada: "AMB-3931".to_string(),
        tokens_privados: 3000.0,
        aportacion_social: 600.0,
        esta_bloqueado: false,
    });

    println!("Censo IMET activo: {} conductores validados en la red.", red_amb.nodos.len());
    println!("--------------------------------------------------");

    // PRUEBA: Tú solicitas un gasto con tu carnet personal y el jefe da el visto bueno
    println!("Vicente (Carnet: IMET-5800) solicita 200 TEU en el taller de guardia.");
    match red_amb.procesar_gasto_con_consenso("IMET-5800", "IMET-0001", 200.0, true, true) {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("{}", err),
    }

    println!("\nComprobación de saldos del coche AMB-1234:");
    println!("-> Turno Vicente (IMET-5800): {} TEU", red_amb.nodos.get("IMET-5800").unwrap().tokens_privados);
    println!("-> Turno Compañero (IMET-9999): {} TEU", red_amb.nodos.get("IMET-9999").unwrap().tokens_privados);
    println!("--------------------------------------------------");
}
