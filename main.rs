// PROYECTO: TAXI SOBERANO - BARCELONA AMB
// "Ayúdanos a regar la flor de la esperanza, desde nuestra humilde aportación"

use std::collections::HashMap;

#[derive(Debug)]
struct NodoTaxi {
    licencia: String,
    ahorro_tokens: f64,
    guardian_asignado: String, // ID del compañero de confianza (ej. el Jefe)
    esta_bloqueado: bool,
}

impl NodoTaxi {
    // Función para liberar fondos (Seguro, Talleres, Combustible)
    fn autorizar_gasto(
        &mut self, 
        importe: f64, 
        firma_biometrica_propia: bool, 
        firma_guardian: bool
    ) -> Result<String, String> {
        
        // El escudo contra "bandidos" remotos: se requieren ambas presencias físicas
        if firma_biometrica_propia && firma_guardian {
            if self.ahorro_tokens >= importe {
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
        self.ahorro_tokens -= importe;
    }
}

fn main() {
    println!("--- Iniciando Nodo de Honor: Taxi Barcelona ---\n");
    
    // 1. Creamos el Taxi de Vicente con 1000 tokens en su hucha
    let mut taxi_vicente = NodoTaxi {
        licencia: "AMB-1234".to_string(),
        ahorro_tokens: 1000.0,
        guardian_asignado: "Jefe-Nodo-01".to_string(),
        esta_bloqueado: false,
    };

    println!("Estado inicial del taxi: {:?}", taxi_vicente);
    println!("--------------------------------------------------");

    // SIMULACIÓN 1: Vicente va al taller y el Jefe autoriza (Doble firma correcta)
    println!("Simulando operación legítima (Vicente + Guardián)...");
    match taxi_vicente.autorizar_gasto(250.0, true, true) {
        Ok(mensaje) => println!("🟢 ÉXITO: {}", mensaje),
        Err(error) => println!("🔴 ERROR: {}", error),
    }
    println!("Hucha actual de Vicente: {} tokens\n", taxi_vicente.ahorro_tokens);

    // SIMULACIÓN 2: Un intruso intenta sacar dinero a distancia (Falta la firma del Guardián)
    println!("Simulando intento de fraude (Falta firma del Guardián)...");
    match taxi_vicente.autorizar_gasto(100.0, true, false) {
        Ok(mensaje) => println!("🟢 ÉXITO: {}", mensaje),
        Err(error) => println!("🚨 ALERTA DEL SISTEMA: {}", error),
    }
    
    println!("--------------------------------------------------");
    println!("--- Fin de la operación del Nodo ---");
}
