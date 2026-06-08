// PROYECTO: TAXI SOBERANO - BARCELONA AMB
// "Ayúdanos a regar la flor de la esperanza, desde nuestra humilde aportación"

use std::collections::HashMap;
use std::io::{self, Write}; // La herramienta para escuchar tu teclado

#[derive(Debug)]
struct NodoTaxi {
    licencia: String,
    ahorro_tokens: f64,
    guardian_asignado: String, // ID del compañero de confianza
    esta_bloqueado: bool,
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
    println!("--- RED DE HONOR IMET/AMB: CONTROL POR CARNET INDIVIDUAL ---\n");
    let mut red_amb = RedTaxiSoberano::new();

    // Damos de alta a dos conductores para la MISMA licencia (Doble Turno)
    red_amb.registrar_taxista(NodoTaxi {
        carnet_imet: "IMET-64669".to_string(), // Tu carnet personalizado (nacido en el 58)
        licencia_vinculada: "AMB-3931".to_string(), // Tu coche
        tokens_privados: 1000.0,
        aportacion_social: 0.0,
        esta_bloqueado: false,
    });

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
